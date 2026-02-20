use syn::{Expr, ExprLit, Field, Lit, Meta};

pub fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        match c.is_uppercase() {
            true => {
                if i != 0 {
                    out.push('_');
                }
                out.push(c.to_ascii_lowercase());
            }
            false => out.push(c),
        };
    }
    out
}

pub fn get_from(field: &Field) -> Option<String> {
    for attr in &field.attrs {
        match &attr.meta {
            Meta::NameValue(mnv) if mnv.path.is_ident("from") => {
                return match &mnv.value {
                    Expr::Lit(ExprLit { attrs: _, lit: Lit::Str(s) }) => Some(s.value()),
                    _ => None,
                };
            }
            _ => continue,
        };
    }

    return None;
}
