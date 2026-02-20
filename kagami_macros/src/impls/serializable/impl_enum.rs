use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, ExprLit, Fields, FieldsNamed, Lit, Variant};

use crate::Format;
use crate::impls::get_deser_fn;
use crate::impls::get_ser_fn;

fn get_enum_repr(item: &DeriveInput) -> proc_macro2::TokenStream {
    if let Some(ml) = item.attrs.iter().find_map(|attr| match &attr.meta {
        syn::Meta::List(ml) if ml.path.is_ident("my_repr") => Some(ml),
        _ => panic!("No repr on enum"),
    }) {
        return ml.tokens.clone();
    };

    panic!("No repr on enum");
}

fn get_unit_ser(
    variant: &Variant,
    i: usize,
    discriminant: &mut i32,
    repr: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let name = &variant.ident;

    if let Some((_, Expr::Lit(syn::ExprLit { attrs: _, lit: Lit::Int(d) }))) = &variant.discriminant {
        let d = d.base10_parse::<i32>().unwrap();
        *discriminant = d;
    }

    let discriminant = *discriminant + i as i32;

    let format = Format::VarInt;

    let discriminant = match repr.to_string().as_str() {
        "i32" => match format {
            Format::Standard => {
                quote! { raw_payload.write(#discriminant.to_le_bytes())?; }
            }
            Format::VarInt => {
                quote! { raw_payload.write(&crate::varint::temp_convert(#discriminant)?)?; }
            }
            _ => panic!("Unsupported format"),
        },
        _ => panic!("Repr not supported"),
    };

    quote! {
        #name => { #discriminant },
    }
}

fn get_named_ser(
    variant: &Variant,
    fields: &FieldsNamed,
    i: usize,
    discriminant: &mut i32,
    repr: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let name = &variant.ident;

    if let Some((_, Expr::Lit(syn::ExprLit { attrs: _, lit: Lit::Int(d) }))) = &variant.discriminant {
        let d = d.base10_parse::<i32>().unwrap();
        *discriminant = d;
    }

    let discriminant = *discriminant + i as i32;

    let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
    let field_sers = fields.named.iter().map(|field| get_ser_fn(&field, false));

    let format = Format::VarInt;

    let discriminant = match repr.to_string().as_str() {
        "i32" => match format {
            Format::Standard => {
                quote! { raw_payload.write(#discriminant.to_le_bytes())?; }
            }
            Format::VarInt => {
                quote! { raw_payload.write(&crate::varint::temp_convert(#discriminant)?)?; }
            }
            _ => panic!("Unsupported format"),
        },
        _ => panic!("Repr not supported"),
    };

    quote! {
        #name { #( #field_names ),* } => {
            #discriminant
            #( #field_sers )*
        },
    }
}

fn get_variant_ser(
    variant: &Variant,
    i: usize,
    discriminant: &mut i32,
    repr: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match &variant.fields {
        Fields::Unit => get_unit_ser(variant, i, discriminant, repr),
        Fields::Named(fields) => get_named_ser(variant, fields, i, discriminant, repr),
        Fields::Unnamed(_) => panic!("Unnamed fields are not supported in enums"),
    }
}

fn get_variant_de(variant: &Variant, i: usize, discriminant: &mut i32) -> proc_macro2::TokenStream {
    let name = &variant.ident;

    if let Some((_, Expr::Lit(ExprLit { attrs: _, lit: Lit::Int(d) }))) = &variant.discriminant {
        let d = d.base10_parse::<i32>().unwrap();
        *discriminant = d;
    }

    let discriminant = *discriminant + i as i32;

    match &variant.fields {
        Fields::Unit => {
            quote! { #discriminant => { Ok((input, #name)) }, }
        }
        Fields::Named(fields) => {
            let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

            let deserializers = fields.named.iter().map(|field| {
                let name = &field.ident;
                let de = get_deser_fn(field);
                quote! { let (input, #name) = #de }
            });

            quote! {
                #discriminant => {
                    #( #deserializers )*
                    Ok((input, #name { #( #field_names ),* }))
                }
            }
        }
        Fields::Unnamed(_) => {
            panic!("Unnamed fields are not supported in enums");
        }
    }
}

pub fn get_serializable_enum_impl(item: &DeriveInput, data: &syn::DataEnum) -> TokenStream {
    let name = &item.ident;
    let repr = get_enum_repr(item);

    let mut discriminant = 0;

    let generics = &item.generics;
    let (_, ty_generics, _) = generics.split_for_impl();

    let mut serializers = vec![];
    let mut deserializers = vec![];

    for (i, variant) in data.variants.iter().enumerate() {
        serializers.push(get_variant_ser(variant, i, &mut discriminant, &repr));
        deserializers.push(get_variant_de(variant, i, &mut discriminant));
    }

    TokenStream::from(quote! {
        impl<'a> Serializable<'a> for #name #ty_generics {
            fn serialize(&'a self, raw_payload: &'a mut Vec<u8>) -> crate::error::PResult<()> {
                use #name::*;

                match self {
                    #( #serializers )*
                };

                Ok(())
            }

            fn deserialize(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
                let (input, discriminant) = varint_i32(input)?;
                use #name::*;

                match discriminant {
                    #( #deserializers )*
                    _ => panic!("Unknown Variant")
                }
            }
        }
    })
}
