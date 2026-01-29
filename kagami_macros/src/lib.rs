use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

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

#[proc_macro_derive(Payload)]
pub fn derive_payload(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let snake = to_snake_case(&name.to_string());
    let field = Ident::new(&snake, name.span());

    let Data::Struct(data) = &input.data else {
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

    let packet_id = match name.to_string().as_str() {
        "ClientChat" => 0x01,
        "ServerChat" => 0x02,
        _ => 0x02,
    };

    TokenStream::from(quote! {
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

                Ok(Packet { id: #packet_id, raw_payload })
            }

        }
    })
}

#[proc_macro_derive(Dispatch)]
pub fn derive_dispatch(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let snake = to_snake_case(&name.to_string());
    let field = Ident::new(&snake, name.span());

    let (_, ty_generics, where_clause) = generics.split_for_impl();

    TokenStream::from(quote! {
        impl<'a> Dispatch<'a> for #name #ty_generics #where_clause {
            fn dispatch(self, ctx: &mut ProxyContext) -> Option<Self> {
                let mut pctx = Context::new(self, &mut ctx.dst, &mut ctx.src);

                for event in &ctx.proxy.events.packet_events.#field {
                    event(&mut pctx);
                }

                match pctx.cancel {
                    true => None,
                    false => Some(pctx.payload),
                }
            }
        }
    })
}
