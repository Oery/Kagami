use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Data, DeriveInput, Field, Fields, Generics, Type, parse_macro_input};

enum Origin {
    Client,
    Server,
}

impl Origin {
    fn to_string(&self) -> String {
        match self {
            Origin::Client => "client",
            Origin::Server => "server",
        }
        .into()
    }
}

enum State {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

enum Format {
    Standard,
    VarInt,
    JSON,
}

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s {
            "varint" => Format::VarInt,
            "json" => Format::JSON,
            _ => Format::Standard,
        }
    }
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        match c.is_uppercase() {
            true => {
                if i != 0 {
                    out.push('_');
                }
                out.push(c.to_ascii_lowercase());
            }
            false => out.push(c),
        };
    }
    out
}

fn get_format(field: &Field) -> Format {
    if let Some(mnv) = field.attrs.iter().find_map(|attr| match &attr.meta {
        syn::Meta::NameValue(mnv) if mnv.path.is_ident("format") => Some(mnv),
        _ => None,
    }) {
        let syn::Expr::Lit(lit) = &mnv.value else {
            return Format::Standard;
        };

        let syn::Lit::Str(format) = &lit.lit else {
            return Format::Standard;
        };

        return Format::from(format.value().as_ref());
    };

    return Format::Standard;
}

fn get_from(field: &Field) -> Option<String> {
    if let Some(mnv) = field.attrs.iter().find_map(|attr| match &attr.meta {
        syn::Meta::NameValue(mnv) if mnv.path.is_ident("from") => Some(mnv),
        _ => None,
    }) {
        let syn::Expr::Lit(lit) = &mnv.value else {
            return None;
        };

        let syn::Lit::Str(src) = &lit.lit else {
            return None;
        };

        return Some(src.value());
    };

    None
}

fn get_ser_fn(ty: &Type, name: &Ident, format: Format, from: Option<String>) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            let path = &type_path.path;

            // last segment: Vec<T>, Option<T>, String, etc
            let segment = path.segments.last().unwrap();
            let ident = &segment.ident;

            let data_type = match from {
                Some(ref data_type) => data_type.to_owned(),
                None => ident.to_string(),
            };

            let field = match from {
                Some(_) => quote! { value },
                None => quote! { self.#name },
            };

            let data_ser = match data_type.as_str() {
                "bool" => quote::quote! {
                    raw_payload.write(&[#field as u8])?;
                },

                "u8" => quote::quote! {
                    raw_payload.write(&[#field])?;
                },

                "i16" => quote::quote! {
                    raw_payload.write(&#field.to_be_bytes())?;
                },

                "i32" => match format {
                    Format::Standard => {
                        quote::quote! {
                            raw_payload.write(&#field.to_le_bytes())?;
                        }
                    }
                    Format::VarInt => {
                        quote::quote! { raw_payload.write(&crate::varint::temp_convert(#field)?)?; }
                    }
                    _ => panic!("Unsupported format"),
                },

                "String" => match format {
                    Format::Standard => quote::quote! {
                        raw_payload.write(&crate::varint::temp_convert(#field.len() as i32)?)?;
                        raw_payload.write(#field.as_bytes())?;
                    },
                    _ => panic!("Unsupported format"),
                },

                "Cow" => quote::quote! {
                    raw_payload.write(&crate::varint::temp_convert(#field.len() as i32)?)?;
                    raw_payload.write(#field.as_bytes())?;
                },

                "McState" => quote::quote! {
                    raw_payload.write(&crate::varint::temp_convert(#field as i32)?)?;
                },

                _ => match format {
                    Format::JSON => quote::quote! {
                        let j = serde_json::to_string(&#field).unwrap();
                        raw_payload.write(&crate::varint::temp_convert(j.len() as i32)?)?;
                        raw_payload.write(j.as_bytes())?;
                    },
                    _ => panic!("Type '{ident}' has no standard encoder"),
                },
            };

            match from {
                Some(_) => {
                    let ty_ident = Ident::new(&data_type, Span::call_site());
                    quote! {
                        let value = self.#name as #ty_ident;
                        #data_ser
                    }
                }
                None => data_ser,
            }
        }
        _ => quote::quote! {
            Default::default()
        },
    }
}

fn get_deser_fn(ty: &Type, name: &Ident, format: Format, from: Option<String>) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            let path = &type_path.path;

            // last segment: Vec<T>, Option<T>, String, etc
            let segment = path.segments.last().unwrap();
            let ident = &segment.ident;

            let data_type = match from {
                Some(ref data_type) => data_type.to_owned(),
                None => ident.to_string(),
            };

            let data_de = match data_type.as_str() {
                "bool" => quote::quote! {
                    nom::bytes::streaming::take(1usize)(input)?;
                    let #name = #name[0] == 1;
                },

                "u8" => quote::quote! {
                    nom::bytes::streaming::take(1usize)(input)?;
                    let #name = #name[0];
                },

                "i32" => match format {
                    Format::Standard => quote::quote! { int(input)?; },
                    Format::VarInt => quote::quote! { varint_i32(input)?; },
                    _ => panic!("Unsupported format"),
                },

                "i16" => quote::quote! { short(input)?; },

                "String" => match format {
                    Format::Standard => quote::quote! { string(input)?; },
                    _ => panic!("Unsupported format"),
                },

                "Cow" => quote::quote! { string(input)?; },

                "McState" => quote::quote! { state(input)?; },

                _ => match format {
                    Format::JSON => quote::quote! { json::<#ident>(input)?; },
                    _ => panic!("Type '{ident}' has no standard encoder"),
                },
            };

            match from {
                Some(_) => {
                    quote! {
                        {
                            let (input, #name) = #data_de
                            (input, #ident::from_repr(#name).unwrap())
                        };
                    }
                }
                None => data_de,
            }
        }
        _ => quote::quote! {
            Default::default()
        },
    }
}

fn last_path_segment(ty: &Type) -> Option<&syn::PathSegment> {
    match ty {
        Type::Path(p) => p.path.segments.last(),
        _ => None,
    }
}

fn cow_str(ty: &Type) -> Option<()> {
    let seg = last_path_segment(ty)?;

    if seg.ident != "Cow" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };

    let mut it = args.args.iter();

    match it.next()? {
        syn::GenericArgument::Lifetime(_) => {}
        _ => return None,
    }

    match it.next()? {
        syn::GenericArgument::Type(syn::Type::Path(p)) if p.path.is_ident("str") => {}
        _ => return None,
    }

    Some(())
}

fn is_cow_str(ty: &Type) -> bool {
    cow_str(ty).is_some()
}

fn get_payload_impl(item: &DeriveInput, origin: &Origin) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let generics = &item.generics;

    let snake_name = to_snake_case(&name.to_string());
    let snake_orig = to_snake_case(&origin.to_string());
    let field = Ident::new(&format!("{snake_orig}_{snake_name}"), name.span());

    let (_, ty_generics, _) = generics.split_for_impl();

    let item_type = match generics.lifetimes().next().is_some() {
        true => quote! { #name<'b> },
        false => quote! { #name },
    };

    quote! {
        impl<'a> crate::proxy::Payload<'a> for #name #ty_generics {
            type Item<'b> = #item_type;
            type Handler = Box<dyn for<'b> Fn(&mut Context<#item_type>) + Send + Sync + 'static>;

            fn register(em: &mut EventManager, f: Self::Handler) {
                em.packet_events.#field.push(f);
            }

            fn has_events(em: &EventManager) -> bool {
                !em.packet_events.#field.is_empty()
            }
        }
    }
}

fn get_dispatch_impl(name: &Ident, origin: &Origin, generics: &Generics) -> proc_macro2::TokenStream {
    let snake_name = to_snake_case(&name.to_string());
    let snake_orig = to_snake_case(&origin.to_string());
    let field = Ident::new(&format!("{snake_orig}_{snake_name}"), name.span());

    let (_, ty_generics, _) = generics.split_for_impl();

    let new_pctx = match origin {
        Origin::Client => quote! { Context::new(self, &mut ctx.src, &mut ctx.dst, &ctx.state) },
        Origin::Server => quote! { Context::new(self, &mut ctx.dst, &mut ctx.src, &ctx.state) },
    };

    quote! {
        impl<'a> crate::proxy::Dispatch<'a> for #name #ty_generics {
            fn dispatch(self, ctx: &mut crate::context::ProxyContext) -> Option<Self> {
                let mut pctx = #new_pctx;

                for event in &ctx.proxy.events.packet_events.#field {
                    event(&mut pctx);
                }

                match pctx.cancel {
                    true => None,
                    false => Some(pctx.payload),
                }
            }
        }
    }
}

struct PacketAttributes {
    state: State,
    id: i32,
    origin: Origin,
}

impl Parse for PacketAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let state_ident = input.parse::<syn::Ident>()?;
        let state = match state_ident.to_string().as_str() {
            "Handshake" => State::Handshake,
            "Status" => State::Status,
            "Login" => State::Login,
            "Play" => State::Play,
            _ => panic!("Invalid State"),
        };

        input.parse::<syn::Token![,]>()?;

        let id_lit = input.parse::<syn::LitInt>()?;
        let id = id_lit.base10_parse::<i32>()?;

        input.parse::<syn::Token![,]>()?;

        let origin_ident = input.parse::<syn::Ident>()?;
        let origin = match origin_ident.to_string().as_str() {
            "Client" => Origin::Client,
            "Server" => Origin::Server,
            _ => panic!("Invalid Origin"),
        };

        Ok(PacketAttributes { state, id, origin })
    }
}

fn get_origin_impl(name: &Ident, origin: &Origin, generics: &Generics) -> proc_macro2::TokenStream {
    let item_type = match generics.lifetimes().next().is_some() {
        true => quote! { #name<'a> },
        false => quote! { #name },
    };

    match origin {
        Origin::Client => quote! { impl<'a> ClientPacket<'a> for #item_type {} },
        Origin::Server => quote! { impl<'a> ServerPacket<'a> for #item_type {} },
    }
}

#[proc_macro_derive(Serializable, attributes(format, from))]
pub fn serializable(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let name = &item.ident;

    let (_, ty_generics, _) = &item.generics.split_for_impl();

    let Data::Struct(data) = &item.data else {
        panic!("This is not a struct");
    };

    let Fields::Named(fields) = &data.fields else {
        panic!("Fields should be named");
    };

    let field_desers = fields.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let format = get_format(field);
        let from = get_from(field);
        let deser_fn = get_deser_fn(&field.ty, name, format, from);

        quote! {
            let (input, #name) = #deser_fn
        }
    });

    let field_sers = fields.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let format = get_format(field);
        let from = get_from(field);
        get_ser_fn(&field.ty, name, format, from)
    });

    let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

    let deserializer = quote! {
        fn deserialize(input: &'a [u8]) -> crate::error::PResult<Self> {
            #( #field_desers )*

            Ok(Self { #( #field_names ),* })
        }
    };

    TokenStream::from(quote! {
        impl<'a> crate::proxy::Serializable<'a> for #name #ty_generics {
            #deserializer

            fn serialize(&self) -> crate::error::PResult<Packet<'_>> {
                let mut raw_payload = vec![];
                #( #field_sers )*
                let raw_payload: Cow<'_, [u8]> = raw_payload.into();

                Ok(Packet { id: self.id(), raw_payload })
            }
        }
    })
}

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, input: TokenStream) -> TokenStream {
    let PacketAttributes { state, id, origin } = parse_macro_input!(attr as PacketAttributes);
    let derive_input = input.clone();
    let item = parse_macro_input!(derive_input as DeriveInput);
    let name = &item.ident;
    let generics = &item.generics;

    let (_, ty_generics, _) = generics.split_for_impl();

    let impl_payload = get_payload_impl(&item, &origin);
    let impl_origin = get_origin_impl(name, &origin, generics);
    let impl_dispatch = get_dispatch_impl(name, &origin, generics);

    TokenStream::from(quote! {
        #[derive(Serializable, Debug)]
        #item

        impl<'a> #name #ty_generics {
            pub fn id(&self) -> i32 {
                #id
            }
        }

        #impl_payload

        #impl_dispatch

        #impl_origin
    })
}
