use syn::{Attribute, Expr, ExprLit, Lit, Meta};

pub enum Format {
    Standard,
    VarInt,
}

impl Format {
    pub fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut formats = attrs.iter().filter(|attr| attr.path().is_ident("format"));
        let Some(attr) = formats.next() else {
            return Ok(Self::Standard);
        };
        if let Some(duplicate) = formats.next() {
            return Err(syn::Error::new_spanned(duplicate, "duplicate field format"));
        }
        if let Meta::NameValue(value) = &attr.meta
            && let Expr::Lit(ExprLit { lit: Lit::Str(name), .. }) = &value.value
        {
            return match name.value().as_str() {
                "varint" => Ok(Self::VarInt),
                _ => Err(syn::Error::new_spanned(name, "expected varint")),
            };
        }
        Err(syn::Error::new_spanned(attr, "expected #[format = \"varint\"]"))
    }
}

#[cfg(test)]
#[path = "tests/tests_format.rs"]
mod tests;
