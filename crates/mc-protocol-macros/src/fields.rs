use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{DataStruct, Field, Fields, Type};

use crate::format::Format;

/// The generated codec of a struct's named fields, in declaration order.
pub struct FieldCodecs {
    /// One `let field = ...;` statement per field, reading from `buf`.
    pub decodes: Vec<Tokens>,
    /// One statement per field, appending `self.field` to `out`.
    pub encodes: Vec<Tokens>,
    /// One `usize` expression per field.
    pub lengths: Vec<Tokens>,
    /// `Self { .. }` built from the decoded fields, or `Self` for a unit struct.
    pub construct: Tokens,
}

/// Generates the codec of every field. `what` names the item in errors,
/// such as "packets".
pub fn field_codecs(data: &DataStruct, what: &str) -> syn::Result<FieldCodecs> {
    let fields: Vec<&Field> = match &data.fields {
        Fields::Named(fields) => fields.named.iter().collect(),
        Fields::Unit => Vec::new(),
        Fields::Unnamed(_) => {
            return Err(syn::Error::new_spanned(&data.fields, format!("{what} require named fields")));
        }
    };

    let mut decodes = Vec::new();
    let mut encodes = Vec::new();
    let mut lengths = Vec::new();

    for field in &fields {
        let ident = &field.ident;
        let ty = &field.ty;
        let (decode, encode, length) = match Format::parse(&field.attrs)? {
            Format::Standard => (
                quote!(let #ident = <#ty as ::mc_protocol::PacketWriter>::read(buf)?;),
                quote!(::mc_protocol::PacketWriter::encode(&self.#ident, out)?;),
                quote!(::mc_protocol::PacketWriter::body_len(&self.#ident)),
            ),
            Format::VarInt => variable_length(ident, ty, quote!(::mc_protocol::VarInt), quote!(i32)),
            Format::VarLong => variable_length(ident, ty, quote!(::mc_protocol::types::VarLong), quote!(i64)),
        };

        decodes.push(decode);
        encodes.push(encode);
        lengths.push(length);
    }

    let names = fields.iter().map(|field| &field.ident);
    let construct = if matches!(data.fields, Fields::Unit) {
        quote!(Self)
    } else {
        quote!(Self { #(#names),* })
    };

    Ok(FieldCodecs { decodes, encodes, lengths, construct })
}

/// The codec of a field sent as a VarInt or VarLong: `wrapper` is the
/// codec type and `wire` the integer it holds.
fn variable_length(ident: &Option<syn::Ident>, ty: &Type, wrapper: Tokens, wire: Tokens) -> (Tokens, Tokens, Tokens) {
    // A field of the wire type needs no fallible conversion; other integer
    // fields keep their checked conversion from the signed wire value.
    let read = quote!(<#wrapper as ::mc_protocol::PacketWriter>::read(buf)?.0);
    let wire_name = wire.to_string();
    let is_wire_type = match ty {
        Type::Path(path) => path.path.segments.last().is_some_and(|segment| segment.ident == wire_name),
        _ => false,
    };
    let value = if is_wire_type { read } else { quote!(#read.try_into()?) };

    let encoded = quote!(#wrapper(self.#ident as #wire));
    let decode = quote!(let #ident: #ty = #value;);
    let encode = quote!(::mc_protocol::PacketWriter::encode(&#encoded, out)?;);
    let length = quote!(::mc_protocol::PacketWriter::body_len(&#encoded));

    (decode, encode, length)
}
