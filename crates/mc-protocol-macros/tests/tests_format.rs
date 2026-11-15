use super::*;

#[test]
fn documentation_does_not_hide_a_field_format() {
    let attrs = vec![syn::parse_quote!(#[doc="An optional entity ID"]), syn::parse_quote!(#[format="varint"])];
    assert!(matches!(Format::parse(&attrs).unwrap(), Format::VarInt));
}

#[test]
fn unknown_duplicate_and_malformed_formats_are_rejected() {
    for attrs in [
        vec![syn::parse_quote!(#[format = "varnit"])],
        vec![syn::parse_quote!(#[format = "varint"]), syn::parse_quote!(#[format = "varint"])],
        vec![syn::parse_quote!(#[format])],
        vec![syn::parse_quote!(#[format = 3])],
        vec![syn::parse_quote!(#[format(json)])],
    ] {
        assert!(Format::parse(&attrs).is_err());
    }

    assert!(matches!(Format::parse(&[]).unwrap(), Format::Standard));
}
