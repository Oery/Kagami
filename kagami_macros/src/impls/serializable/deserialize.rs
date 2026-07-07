use quote::quote;
use syn::{Field, Type};

use crate::format::*;
use crate::utils::get_enum_repr;

pub fn get_deser_fn(field: &Field) -> proc_macro2::TokenStream {
    let Type::Path(type_path) = &field.ty else {
        panic!("Deser Fn panic");
    };

    let format = get_format(field);
    let from = get_enum_repr(field);
    let ty = &field.ty;

    let name = &field.ident;

    let name = match from {
        Some(_) => quote! { discriminant },
        None => quote! { #name },
    };

    // last segment: Vec<T>, Option<T>, String, etc
    let segment = &type_path.path.segments.last().unwrap();
    let ident = &segment.ident;

    let data_type = match from {
        Some(ref data_type) => data_type.to_owned(),
        None => ident.to_string(),
    };

    let data_de = match data_type.as_str() {
        "i16" => quote! { short(input)?; },
        "f32" => quote! { float(input)?; },
        "f64" => quote! { double(input)?; },

        "bool" => quote! {
            nom::bytes::streaming::take(1usize)(input)?;
            let #name = #name[0] == 1;
        },

        "u8" => quote! {
            nom::bytes::streaming::take(1usize)(input)?;
            let #name = #name[0];
        },

        "i8" => quote! {
            nom::bytes::streaming::take(1usize)(input)?;
            let #name = #name[0] as i8;
        },

        "i32" => match format {
            Format::Standard => quote! { int(input)?; },
            Format::VarInt => quote! { varint_i32(input)?; },
            _ => panic!("Unsupported format"),
        },

        "String" => match format {
            Format::Standard => quote! { string(input)?; },
            _ => panic!("Unsupported format"),
        },

        _ => match format {
            Format::Json => quote! { json::<#ident>(input)?; },
            _ => quote! { #ident::deserialize(input)?; },
        },
    };

    let name = &field.ident;

    // Strip Generics
    let mut ty = field.ty.clone();
    if let Type::Path(syn::TypePath { path, .. }) = &mut ty {
        if let Some(last) = path.segments.last_mut() {
            last.arguments = syn::PathArguments::None;
        }
    }

    match from {
        Some(_) => {
            quote! {
                {
                    let (input, discriminant) = #data_de
                    let (input, #name) = #ty :: deserialize_enum(input, discriminant as i32)?;
                    (input, #name)
                };
            }
        }
        None => data_de,
    }
}
