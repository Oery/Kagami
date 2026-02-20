use quote::quote;
use syn::{Field, Type};

use crate::format::*;
use crate::utils::get_from;

pub fn get_deser_fn(field: &Field) -> proc_macro2::TokenStream {
    let name = &field.ident;

    let format = get_format(field);
    let from = get_from(field);

    match &field.ty {
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
                "bool" => quote! {
                    nom::bytes::streaming::take(1usize)(input)?;
                    let #name = #name[0] == 1;
                },

                "u8" => quote! {
                    nom::bytes::streaming::take(1usize)(input)?;
                    let #name = #name[0];
                },

                "i32" => match format {
                    Format::Standard => quote! { int(input)?; },
                    Format::VarInt => quote! { varint_i32(input)?; },
                    _ => panic!("Unsupported format"),
                },

                "i16" => quote! { short(input)?; },

                "f32" => quote! { float(input)?; },

                "String" => match format {
                    Format::Standard => quote! { string(input)?; },
                    _ => panic!("Unsupported format"),
                },

                "Cow" => quote! { string(input)?; },

                _ => match format {
                    Format::JSON => quote! { json::<#ident>(input)?; },
                    _ => quote! { #ident::deserialize(input)?; },
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
        _ => quote! {
            Default::default()
        },
    }
}
