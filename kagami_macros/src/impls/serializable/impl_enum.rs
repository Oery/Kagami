use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, ExprLit, Fields, Lit, Variant};

use crate::impls::get_deser_fn;
use crate::impls::get_ser_fn;

fn get_field_discrim(variant: &Variant, discriminant: &mut i32, i: i32) -> proc_macro2::TokenStream {
    let name = &variant.ident;
    let current = *discriminant + i;

    match &variant.fields {
        Fields::Unit => quote! {
            Self :: #name => #current,
        },
        Fields::Named(_) => quote! {
            Self :: #name { .. }  => #current,
        },
        Fields::Unnamed(_) => panic!("Unnamed fields are not supported in enums"),
    }
}

fn get_variant_ser(variant: &Variant, discriminant: &mut i32) -> proc_macro2::TokenStream {
    let name = &variant.ident;

    if let Some((_, expr)) = &variant.discriminant {
        let d = match expr {
            Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) => lit.base10_parse::<i32>().unwrap(),
            Expr::Unary(syn::ExprUnary { op: syn::UnOp::Neg(_), expr, .. }) => {
                if let Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) = &**expr {
                    -lit.base10_parse::<i32>().unwrap()
                } else {
                    panic!("Unsupported negative expression")
                }
            }
            _ => panic!("Unsupported discriminant expression"),
        };
        *discriminant = d;
    }

    match &variant.fields {
        Fields::Unit => quote! {
            Self :: #name => {},
        },
        Fields::Named(fields) => {
            let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
            let field_sers = fields.named.iter().map(|f| get_ser_fn(f, false));

            quote! {
                Self :: #name { #( #field_names ),* } => {
                    #( #field_sers )*
                },
            }
        }
        Fields::Unnamed(_) => panic!("Unnamed fields are not supported in enums"),
    }
}

fn get_variant_de(variant: &Variant, discriminant: &mut i32, i: i32) -> proc_macro2::TokenStream {
    let name = &variant.ident;

    let mut current = *discriminant + i;

    if let Some((_, expr)) = &variant.discriminant {
        let d = match expr {
            Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) => lit.base10_parse::<i32>().unwrap(),
            Expr::Unary(syn::ExprUnary { op: syn::UnOp::Neg(_), expr, .. }) => {
                if let Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) = &**expr {
                    -lit.base10_parse::<i32>().unwrap()
                } else {
                    panic!("Unsupported negative expression")
                }
            }
            _ => panic!("Unsupported discriminant expression"),
        };
        *discriminant = d;
        current = d;
    }

    match &variant.fields {
        Fields::Unit => {
            quote! { #current => { Ok((input, Self :: #name)) }, }
        }
        Fields::Named(fields) => {
            let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

            let deserializers = fields.named.iter().map(|field| {
                let name = &field.ident;
                let de = get_deser_fn(field);
                quote! { let (input, #name) = #de }
            });

            quote! {
                #current => {
                    #( #deserializers )*
                    Ok((input, Self :: #name { #( #field_names ),* }))
                },
            }
        }
        Fields::Unnamed(_) => {
            panic!("Unnamed fields are not supported in enums");
        }
    }
}

pub fn get_serializable_enum_impl(item: &DeriveInput, data: &syn::DataEnum) -> TokenStream {
    let name = &item.ident;

    let mut discriminant = 0;

    let generics = &item.generics;
    let (_, ty_generics, _) = generics.split_for_impl();

    let mut serializers = vec![];
    let mut deserializers = vec![];
    let mut field_to_discriminant = vec![];

    for (i, variant) in data.variants.iter().enumerate() {
        serializers.push(get_variant_ser(variant, &mut discriminant));
        deserializers.push(get_variant_de(variant, &mut discriminant, i as i32));
        field_to_discriminant.push(get_field_discrim(variant, &mut discriminant, i as i32));
    }

    let name_as_str = name.to_string();

    TokenStream::from(quote! {
        impl<'a> Serializable<'a> for #name #ty_generics {
            fn serialize(&'a self, raw_payload: &'a mut Vec<u8>) -> crate::error::PResult<()> {
                panic!("Enum should not be serialized with this")
            }

            fn deserialize(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
                panic!("Enum should not be deserialized with this")
            }
        }

        impl<'a> #name #ty_generics {
            pub fn serialize_enum(&'a self, raw_payload: &'a mut Vec<u8>) -> crate::error::PResult<()> {
                match self {
                    #( #serializers )*
                    _ => panic!("Unknown Variant for {}: {:?}", #name_as_str, self),
                };

                Ok(())
            }

            pub fn deserialize_enum(input: &'a [u8], discriminant: i32) -> nom::IResult<&'a [u8], Self> {
                match discriminant {
                    #( #deserializers )*
                    _ => panic!("Unknown Variant for {}: {discriminant}", #name_as_str),
                }
            }

            pub fn to_u8(&self) -> u8 {
                let val = self.to_i32();

                val as u8
            }

            pub fn to_i32(&self) -> i32 {
                match self {
                    #( #field_to_discriminant )*
                    _ => panic!("Unknown Variant"),
                }
            }

            pub fn to_f32(&self) -> f32 {
                let val = self.to_i32();

                val as f32
            }
        }
    })
}
