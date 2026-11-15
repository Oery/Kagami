use super::*;

#[test]
fn invalid_packet_attributes_and_declarations_have_specific_diagnostics() {
    for (attributes, expected) in [
        (quote!(Unknown, 0x00, Client), "unknown packet state"),
        (quote!(Play, 0x00, Unknown), "expected Client or Server"),
        (quote!(Play, 0x00, Client, extra), "unexpected token"),
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
        let attributes = syn::parse2(quote!(Play, 0x00, Client)).unwrap();
        let error = expand_packet(attributes, &input).unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
    }
}

#[test]
fn inner_packet_rejects_invalid_variant_shapes() {
    for input in [
        syn::parse_quote!(
            struct Bad;
        ),
        syn::parse_quote!(
            enum Bad {
                Unit,
            }
        ),
        syn::parse_quote!(
            enum Bad {
                Pair(A, B),
            }
        ),
        syn::parse_quote!(
            enum Bad {
                Empty(),
            }
        ),
        syn::parse_quote!(
            enum Bad {
                Named { value: A },
            }
        ),
    ] {
        assert!(expand_inner_packet(&input).is_err());
    }
}
