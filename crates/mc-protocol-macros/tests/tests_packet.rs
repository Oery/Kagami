use super::*;

#[test]
fn invalid_packet_attributes_and_declarations_have_specific_diagnostics() {
    for (attributes, expected) in [
        (quote!(Unknown, 0x00, Clientbound), "unknown packet state"),
        (quote!(Play, 0x00, Unknown), "expected Serverbound or Clientbound"),
        (quote!(Play, 0x100, Clientbound), "packet ids must be in 0..=255"),
        (quote!(Play, -1, Clientbound), "packet ids must be in 0..=255"),
        (quote!(Play, 0x00, Clientbound, extra), "unexpected token"),
    ] {
        let error = syn::parse2::<PacketAttributes>(attributes).err().unwrap();
        assert!(error.to_string().contains(expected), "{error}");
    }
    for (input, expected) in [
        (
            syn::parse_quote!(
                enum Bad {}
            ),
            "must be structs",
        ),
        (
            syn::parse_quote!(
                struct Bad(i32);
            ),
            "named fields",
        ),
        (
            syn::parse_quote!(
                struct Bad<T> {
                    value: T,
                }
            ),
            "no type or const parameters",
        ),
        (
            syn::parse_quote!(
                struct Bad<'a, 'b> {
                    value: &'a str,
                    other: &'b str,
                }
            ),
            "at most one lifetime",
        ),
    ] {
        let attributes = syn::parse2(quote!(Play, 0x00, Clientbound)).unwrap();
        let error = expand_packet(attributes, &input).unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
    }
}

#[test]
fn packet_enums_reject_invalid_shapes() {
    for (direction, input, expected) in [
        (
            quote!(Sideways),
            quote!(
                enum Bad<'p> {
                    A(A<'p>),
                }
            ),
            "expected Serverbound or Clientbound",
        ),
        (
            quote!(Serverbound),
            quote!(
                enum Bad {
                    A(A),
                }
            ),
            "exactly one lifetime",
        ),
        (
            quote!(Serverbound),
            quote!(
                enum Bad<'p> {
                    A,
                }
            ),
            "exactly one packet",
        ),
        (
            quote!(Serverbound),
            quote!(
                enum Bad<'p> {
                    A(A<'p>, B),
                }
            ),
            "exactly one packet",
        ),
        (
            quote!(Serverbound),
            quote!(
                enum Bad<'p> {
                    A { packet: A<'p> },
                }
            ),
            "exactly one packet",
        ),
    ] {
        let direction = syn::parse2(direction).unwrap();
        let input = syn::parse2(input).unwrap();
        let error = expand_packet_enum(direction, &input).unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
    }
}
