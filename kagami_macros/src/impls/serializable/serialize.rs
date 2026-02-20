use proc_macro2::Span;
use quote::quote;
use syn::{Field, Ident, Type};

use crate::format::*;
use crate::utils::get_from;

pub fn get_ser_fn(field: &Field, is_struct_field: bool) -> proc_macro2::TokenStream {
    let name = &field.ident;

    let format = get_format(field);
    let from = get_from(field);

    let Type::Path(type_path) = &field.ty else {
        panic!();
    };

    let path = &type_path.path;

    // last segment: Vec<T>, Option<T>, String, etc
    let segment = path.segments.last().unwrap();
    let ident = &segment.ident;

    let data_type = match from {
        Some(ref data_type) => data_type.to_owned(),
        None => ident.to_string(),
    };

    let field = match (is_struct_field, &from) {
        (true, None) => quote! { self.#name },
        (true, Some(_)) => quote! { value },
        (false, _) => quote! { #name },
    };

    let field_own = match is_struct_field {
        true => quote! { #field },
        false => quote! { (*#field) },
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
                quote! { raw_payload.write(&#field.to_le_bytes())?; }
            }
            Format::VarInt => {
                quote! { raw_payload.write(&crate::varint::temp_convert(#field_own)?)?; }
            }
            _ => panic!("Unsupported format"),
        },

        "f32" => quote! {
            raw_payload.write(&#field.to_be_bytes())?;
        },

        "String" => match format {
            Format::Standard => quote! {
                raw_payload.write(&crate::varint::temp_convert(#field.len() as i32)?)?;
                raw_payload.write(#field.as_bytes())?;
            },
            _ => panic!("Unsupported format"),
        },

        "Cow" => quote! {
            raw_payload.write(&crate::varint::temp_convert(#field.len() as i32)?)?;
            raw_payload.write(#field.as_bytes())?;
        },

        _ => match format {
            Format::JSON => quote! {
                let j = serde_json::to_string(&#field).unwrap();
                raw_payload.write(&crate::varint::temp_convert(j.len() as i32)?)?;
                raw_payload.write(j.as_bytes())?;
            },
            _ => quote! { #field_own.serialize(raw_payload)?; },
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
