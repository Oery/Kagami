use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

mod format;
mod impls;
mod packet_attrs;
mod utils;

use impls::*;
use packet_attrs::*;

#[proc_macro_derive(SerializablePacket, attributes(format, from))]
pub fn serializable(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let name = &item.ident;

    let Data::Struct(_) = &item.data else {
        panic!("Packet should be a struct");
    };

    let (_, ty_generics, _) = &item.generics.split_for_impl();

    TokenStream::from(quote! {
        impl<'a> SerializablePacket<'a> for #name #ty_generics {
            fn serialize_packet(&self) -> crate::error::PResult<Packet<'_>> {
                let mut raw_payload = vec![];
                self.serialize(&mut raw_payload)?;
                let raw_payload: Cow<'_, [u8]> = raw_payload.into();

                Ok(Packet { id: self.id(), raw_payload })
            }

            fn deserialize_packet(input: &'a [u8]) -> crate::error::PResult<Self> {
                let (input, payload) = Self::deserialize(input)?;

                Ok(payload)
            }
        }
    })
}

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, input: TokenStream) -> TokenStream {
    let PacketAttributes { id, origin } = parse_macro_input!(attr as PacketAttributes);
    let derive_input = input.clone();
    let item = parse_macro_input!(derive_input as DeriveInput);
    let name = &item.ident;

    let (_, ty_generics, _) = &item.generics.split_for_impl();

    let impl_payload = get_payload_impl(&item, &origin);
    let impl_origin = get_origin_impl(name, &origin, &item.generics);
    let impl_dispatch = get_dispatch_impl(name, &origin, &item.generics);

    TokenStream::from(quote! {
        #[derive(Debug, Serializable, SerializablePacket)]
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

#[proc_macro_derive(Serializable, attributes(format, enum_as))]
pub fn serializable_data(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);

    match &item.data {
        Data::Struct(data) => get_serializable_struct_impl(&item, data),
        Data::Enum(data) => get_serializable_enum_impl(&item, data),
        Data::Union(_) => panic!("Unions are not supported"),
    }
}
