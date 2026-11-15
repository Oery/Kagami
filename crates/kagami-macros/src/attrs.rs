//! The `#[command]`, `#[arg]` and `#[value]` attributes, and the names they
//! give.

use std::collections::HashSet;

use proc_macro2::Ident;
use syn::{Attribute, LitStr, Path};

/// What one item's attributes of a namespace say.
#[derive(Default)]
pub struct Attrs {
    pub name: Option<LitStr>,
    pub aliases: Vec<LitStr>,
    pub default: bool,
    pub into: Option<Path>,
}

/// Reads the `namespace` attributes of an item, which may only use the
/// `allowed` keys.
pub fn parse(attrs: &[Attribute], namespace: &str, allowed: &[&str]) -> syn::Result<Attrs> {
    let mut parsed = Attrs::default();

    for attr in attrs {
        if !attr.path().is_ident(namespace) {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            let Some(key) = allowed.iter().find(|key| meta.path.is_ident(key)) else {
                return Err(meta.error(format!("expected one of: {}", allowed.join(", "))));
            };

            match *key {
                "name" => {
                    if parsed.name.is_some() {
                        return Err(meta.error("duplicate name"));
                    }

                    parsed.name = Some(meta.value()?.parse()?);
                }
                "alias" => parsed.aliases.push(meta.value()?.parse()?),
                "default" => parsed.default = true,
                "into" => {
                    if parsed.into.is_some() {
                        return Err(meta.error("duplicate conversion target"));
                    }

                    parsed.into = Some(meta.value()?.parse()?);
                }
                _ => unreachable!("every allowed key is handled"),
            }

            Ok(())
        })?;
    }

    Ok(parsed)
}

/// The name an item is typed as: its `name`, or its identifier in lowercase.
pub fn name(attrs: &Attrs, ident: &Ident) -> syn::Result<LitStr> {
    let name = match &attrs.name {
        Some(name) => name.clone(),
        None => {
            let text = ident.to_string();
            let lowercase = text.trim_start_matches("r#").to_ascii_lowercase();

            LitStr::new(&lowercase, ident.span())
        }
    };

    check_word(&name)?;

    Ok(name)
}

/// Names are single words the client accepts unquoted.
pub fn check_word(name: &LitStr) -> syn::Result<()> {
    let text = name.value();

    let valid = text.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'+'));
    if text.is_empty() || !valid {
        return Err(syn::Error::new_spanned(name, "names must be one word of letters, digits or _-.+"));
    }

    Ok(())
}

/// Fails on a name already in `seen`, then adds it.
pub fn unique(name: &LitStr, seen: &mut HashSet<String>) -> syn::Result<()> {
    if !seen.insert(name.value()) {
        return Err(syn::Error::new_spanned(name, "this name is already taken"));
    }

    Ok(())
}
