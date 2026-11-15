use super::*;

fn args_error(input: DeriveInput) -> String {
    args::expand_args(&input).unwrap_err().to_string()
}

#[test]
fn malformed_commands_explain_what_is_wrong() {
    for (input, expected) in [
        (
            syn::parse_quote! {
                struct Bad { optional: Option<i32>, required: i32 }
            },
            "required arguments cannot follow optional ones",
        ),
        (syn::parse_quote! { struct Bad(i32); }, "arguments are named fields"),
        (syn::parse_quote! { struct Bad<T> { value: T } }, "no type or const parameters"),
        (syn::parse_quote! { struct Bad { #[arg(unknown = "x")] value: i32 } }, "expected one of: name"),
        (
            syn::parse_quote! {
                struct Bad { #[arg(name = "same")] a: i32, #[arg(name = "same")] b: i32 }
            },
            "already taken",
        ),
        (syn::parse_quote! { struct Bad { #[arg(name = "two words")] value: i32 } }, "one word"),
        (syn::parse_quote! { enum Bad {} }, "at least one subcommand"),
        (syn::parse_quote! { enum Bad { Pair(A, B) } }, "exactly one CommandArgs type"),
        (
            syn::parse_quote! {
                enum Bad { #[command(default)] A, #[command(default)] B }
            },
            "only one subcommand can be the default",
        ),
        (syn::parse_quote! { enum Bad { #[command(default, name = "a")] A } }, "has no name"),
        (syn::parse_quote! { enum Bad { A, #[command(name = "a")] B } }, "already taken"),
    ] {
        let error = args_error(input);
        assert!(error.contains(expected), "{error:?} does not say {expected:?}");
    }
}

#[test]
fn malformed_values_explain_what_is_wrong() {
    for (input, expected) in [
        (syn::parse_quote! { struct Bad; }, "derived for enums"),
        (syn::parse_quote! { enum Bad { A(i32) } }, "have no fields"),
        (syn::parse_quote! { enum Bad { A, #[value(alias = "a")] B } }, "already taken"),
        (syn::parse_quote! { enum Bad { #[value(default)] A } }, "expected one of: name, alias"),
    ] {
        let error = value::expand(&input).unwrap_err().to_string();
        assert!(error.contains(expected), "{error:?} does not say {expected:?}");
    }
}
