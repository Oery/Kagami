use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Data, DeriveInput, Fields, Generics, Type, parse_macro_input};

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

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn get_ser_fn(ty: &Type, name: &Ident) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            let path = &type_path.path;

            // last segment: Vec<T>, Option<T>, String, etc
            let segment = path.segments.last().unwrap();
            let ident = &segment.ident;

            match ident.to_string().as_str() {
                "u8" => quote::quote! {
                    raw_payload.write(&[self.#name])?;
                },
                "i32" => quote::quote! {
                    raw_payload.write(&temp_convert(self.#name)?)?;
                },
                "String" => quote::quote! {
                    raw_payload.write(&temp_convert(self.#name.len() as i32)?)?;
                    raw_payload.write(self.#name.as_bytes())?;
                },
                "Cow" => quote::quote! {
                    raw_payload.write(&temp_convert(self.#name.len() as i32)?)?;
                    raw_payload.write(self.#name.as_bytes())?;
                },

                _ => quote::quote! {
                    #ident
                },
            }
        }
        _ => quote::quote! {
            Default::default()
        },
    }
}

fn get_deser_fn(ty: &Type, name: &Ident) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            let path = &type_path.path;

            // last segment: Vec<T>, Option<T>, String, etc
            let segment = path.segments.last().unwrap();
            let ident = &segment.ident;

            match ident.to_string().as_str() {
                "u8" => quote::quote! {
                    nom::bytes::streaming::take(1usize)(input)?;
                    let #name = #name[0];
                },
                "i32" => quote::quote! { varint_i32(input)?; },
                "String" => quote::quote! { string(input)?; },
                "Cow" => quote::quote! { string(input)?; },

                _ => quote::quote! {
                    #ident
                },
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

fn get_payload_impl(item: &DeriveInput, origin: &Origin, id: i32) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let data = &item.data;
    let generics = &item.generics;

    let snake_name = to_snake_case(&name.to_string());
    let snake_orig = to_snake_case(&origin.to_string());
    let field = Ident::new(&format!("{snake_orig}_{snake_name}"), name.span());

    let Data::Struct(data) = data else {
        panic!("This is not a struct");
    };

    let Fields::Named(fields) = &data.fields else {
        panic!("Fields should be named");
    };

    let field_desers = fields.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let deser_fn = get_deser_fn(&field.ty, name);
        quote! {
            let (input, #name) = #deser_fn
        }
    });

    let field_sers = fields.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        get_ser_fn(&field.ty, name)
    });

    let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

    let deserializer = quote! {
        fn deserialize(input: &'a [u8]) -> PResult<Self> {
            #( #field_desers )*

            Ok(Self { #( #field_names ),* })
        }
    };

    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let item_type = match generics.lifetimes().next().is_some() {
        true => quote! { #name<'b> },
        false => quote! { #name },
    };

    quote! {
        impl<'a> Payload<'a> for #name #ty_generics #where_clause {
            type Item<'b> = #item_type;
            type Handler = Box<dyn for<'b> Fn(&mut Context<#item_type>) + Send + Sync + 'static>;

            fn register(em: &mut EventManager, f: Self::Handler) {
                em.packet_events.#field.push(f);
            }

            fn has_events(em: &EventManager) -> bool {
                !em.packet_events.#field.is_empty()
            }

            #deserializer

            fn serialize(&self) -> PResult<Packet<'_>> {
                let mut raw_payload = vec![];
                #( #field_sers )*
                let raw_payload: Cow<'_, [u8]> = raw_payload.into();

                Ok(Packet { id: #id, raw_payload })
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
        Origin::Client => quote! { Context::new(self, &mut ctx.src, &mut ctx.dst) },
        Origin::Server => quote! { Context::new(self, &mut ctx.dst, &mut ctx.src) },
    };

    quote! {
        impl<'a> Dispatch<'a> for #name #ty_generics {
            fn dispatch(self, ctx: &mut ProxyContext) -> Option<Self> {
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

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, input: TokenStream) -> TokenStream {
    let PacketAttributes { state, id, origin } = parse_macro_input!(attr as PacketAttributes);
    let derive_input = input.clone();
    let item = parse_macro_input!(derive_input as DeriveInput);
    let name = &item.ident;
    let generics = &item.generics;

    let impl_payload = get_payload_impl(&item, &origin, id);
    let impl_origin = get_origin_impl(name, &origin, generics);
    let impl_dispatch = get_dispatch_impl(name, &origin, generics);

    TokenStream::from(quote! {
        #[derive(Debug)]
        #item

        #impl_payload

        #impl_dispatch

        #impl_origin
    })
}
