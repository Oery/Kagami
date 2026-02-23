use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Fields};

use crate::impls::get_deser_fn;
use crate::impls::get_ser_fn;

pub fn get_serializable_struct_impl(item: &DeriveInput, data: &DataStruct) -> TokenStream {
    let name = &item.ident;

    let Fields::Named(fields) = &data.fields else {
        panic!("Fields should be named");
    };

    let field_desers = fields.named.iter().map(|field| {
        let name = &field.ident;
        let deser_fn = get_deser_fn(&field);
        quote! { let (input, #name) = #deser_fn }
    });

    let field_sers = fields.named.iter().map(|field| get_ser_fn(field, true));
    let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

    let generics = &item.generics;
    let (_, ty_generics, _) = generics.split_for_impl();

    TokenStream::from(quote! {
        impl<'a> Serializable<'a> for #name #ty_generics {
            fn serialize(&'a self, raw_payload: &'a mut Vec<u8>) -> crate::error::PResult<()> {
                #( #field_sers )*

                Ok(())
            }

            fn deserialize(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
                #( #field_desers )*

                Ok((input, Self { #( #field_names ),* }))
            }
        }
    })
}
