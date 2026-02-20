use syn::{Expr, ExprLit, Field, Lit, Meta};

pub enum Format {
    Standard,
    VarInt,
    JSON,
}

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s {
            "varint" => Format::VarInt,
            "json" => Format::JSON,
            _ => Format::Standard,
        }
    }
}

pub fn get_format(field: &Field) -> Format {
    for attr in &field.attrs {
        match &attr.meta {
            Meta::NameValue(mnv) if mnv.path.is_ident("format") => {
                return match &mnv.value {
                    Expr::Lit(ExprLit { attrs: _, lit: Lit::Str(s) }) => Format::from(s.value().as_str()),
                    _ => Format::Standard,
                };
            }
            _ => continue,
        };
    }

    return Format::Standard;
}
