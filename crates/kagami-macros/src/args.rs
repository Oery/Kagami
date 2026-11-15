//! `#[derive(Command)]` and `#[derive(CommandArgs)]`: positional arguments
//! for structs, subcommands for enums.

use std::collections::HashSet;

use proc_macro2::{Span, TokenStream as Tokens};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DataEnum, DeriveInput, Fields, GenericArgument, GenericParam, Lifetime, PathArguments, Type, Variant};

use crate::attrs;

pub fn expand_command(input: &DeriveInput) -> syn::Result<Tokens> {
    let attrs = attrs::parse(&input.attrs, "command", &["name"])?;
    let name = attrs::name(&attrs, &input.ident)?;

    let args = expand_args(input)?;

    // `on_command::<Remind>` names the command without a lifetime, so the
    // command is the `'static` type and parses into any other.
    let ident = &input.ident;
    let (command, parsed) = match input.generics.lifetimes().next() {
        Some(_) => (quote!(#ident<'static>), quote!(#ident<'__command>)),
        None => (quote!(#ident), quote!(#ident)),
    };

    Ok(quote! {
        #args

        impl ::kagami::command::Command for #command {
            const NAME: &'static str = #name;

            type Parsed<'__command> = #parsed;
        }
    })
}

pub fn expand_args(input: &DeriveInput) -> syn::Result<Tokens> {
    let lifetime = lifetime(input)?;

    let (syntax, parse) = match &input.data {
        Data::Struct(data) => positional(&data.fields, quote!(Self), &lifetime)?,
        Data::Enum(data) => subcommands(input, data, &lifetime)?,
        Data::Union(_) => return Err(syn::Error::new_spanned(input, "commands are structs or enums")),
    };

    let ident = &input.ident;
    let (_, type_generics, _) = input.generics.split_for_impl();

    Ok(quote! {
        impl<#lifetime> ::kagami::command::CommandArgs<#lifetime> for #ident #type_generics {
            const SYNTAX: ::kagami::command::Syntax = #syntax;

            fn parse(
                __words: &mut ::kagami::command::Words<#lifetime>,
            ) -> ::core::result::Result<Self, ::kagami::command::ArgumentError> {
                #parse
            }
        }
    })
}

/// The lifetime arguments borrow from the command: the type's own, or a
/// fresh one for types that borrow nothing.
fn lifetime(input: &DeriveInput) -> syn::Result<Lifetime> {
    let generics = &input.generics;

    let only_lifetimes = generics.params.iter().all(|param| matches!(param, GenericParam::Lifetime(_)));
    if !only_lifetimes || generics.lifetimes().count() > 1 {
        return Err(syn::Error::new_spanned(
            generics,
            "commands have at most one lifetime and no type or const parameters",
        ));
    }

    let lifetime = match generics.lifetimes().next() {
        Some(param) => param.lifetime.clone(),
        None => Lifetime::new("'__command", Span::call_site()),
    };

    Ok(lifetime)
}

/// The syntax and parser of named fields read in order, building
/// `constructor { fields }`.
fn positional(fields: &Fields, constructor: Tokens, lifetime: &Lifetime) -> syn::Result<(Tokens, Tokens)> {
    if let Fields::Unnamed(fields) = fields {
        return Err(syn::Error::new_spanned(fields, "arguments are named fields"));
    }

    let mut specs = Vec::new();
    let mut initializers = Vec::new();
    let mut seen = HashSet::new();
    let mut after_optional = false;

    for (index, field) in fields.iter().enumerate() {
        let ident = field.ident.as_ref().expect("fields are named");
        let attrs = attrs::parse(&field.attrs, "arg", &["name"])?;
        let name = attrs::name(&attrs, ident)?;
        attrs::unique(&name, &mut seen)?;

        let inner = option_inner(&field.ty)?;
        let optional = inner.is_some();
        if after_optional && !optional {
            return Err(syn::Error::new(field.span(), "required arguments cannot follow optional ones"));
        }
        after_optional |= optional;

        let ty = inner.unwrap_or(&field.ty);
        let argument = quote!(<#ty as ::kagami::command::CommandArgument<#lifetime>>);

        // Greedy text takes every word after it, so nothing can follow it.
        let last = index + 1 == fields.len();
        let kind = if last {
            quote!(#argument::KIND)
        } else {
            quote! {{
                assert!(
                    !matches!(#argument::KIND, ::kagami::command::ArgumentKind::GreedyText),
                    "greedy text must be the last argument",
                );
                #argument::KIND
            }}
        };
        specs.push(quote! {
            ::kagami::command::ArgumentSpec { name: #name, optional: #optional, kind: #kind }
        });

        let value = quote! {
            #argument::parse_remaining(__word, __words)
                .map_err(|message| ::kagami::command::ArgumentError::Invalid { name: #name, message })?
        };
        let initializer = if optional {
            quote! {
                #ident: match __words.next() {
                    ::core::option::Option::Some(__word) => ::core::option::Option::Some(#value),
                    ::core::option::Option::None => ::core::option::Option::None,
                }
            }
        } else {
            quote! {
                #ident: {
                    let __word = __words.next().ok_or(::kagami::command::ArgumentError::Missing(#name))?;
                    #value
                }
            }
        };
        initializers.push(initializer);
    }

    let syntax = quote!(::kagami::command::Syntax::Arguments(&[#(#specs),*]));

    let value = match fields {
        Fields::Unit => constructor,
        _ => quote!(#constructor { #(#initializers),* }),
    };
    let parse = quote! {
        let __value = #value;
        if __words.next().is_some() {
            return ::core::result::Result::Err(::kagami::command::ArgumentError::Extra);
        }

        ::core::result::Result::Ok(__value)
    };

    Ok((syntax, parse))
}

/// `T` for a field of type `Option<T>`.
fn option_inner(ty: &Type) -> syn::Result<Option<&Type>> {
    let Type::Path(path) = ty else {
        return Ok(None);
    };

    let Some(segment) = path.path.segments.last() else {
        return Ok(None);
    };
    if segment.ident != "Option" {
        return Ok(None);
    }

    if let PathArguments::AngleBracketed(arguments) = &segment.arguments
        && arguments.args.len() == 1
        && let GenericArgument::Type(inner) = &arguments.args[0]
    {
        return Ok(Some(inner));
    }

    Err(syn::Error::new_spanned(ty, "expected Option<ArgumentType>"))
}

/// Each variant is a subcommand, chosen by its name as the next word. The
/// default variant has no name and is parsed when no name matches.
fn subcommands(input: &DeriveInput, data: &DataEnum, lifetime: &Lifetime) -> syn::Result<(Tokens, Tokens)> {
    if data.variants.is_empty() {
        return Err(syn::Error::new_spanned(input, "a command needs at least one subcommand"));
    }

    let mut specs = Vec::new();
    let mut arms = Vec::new();
    let mut names = Vec::new();
    let mut default = None;
    let mut seen = HashSet::new();

    for variant in &data.variants {
        let attrs = attrs::parse(&variant.attrs, "command", &["name", "default"])?;
        let (syntax, parse) = subcommand(variant, lifetime)?;

        if attrs.default {
            if attrs.name.is_some() {
                return Err(syn::Error::new_spanned(variant, "the default subcommand has no name"));
            }
            if default.is_some() {
                return Err(syn::Error::new_spanned(variant, "only one subcommand can be the default"));
            }

            specs.push(quote! {
                ::kagami::command::Subcommand { name: ::core::option::Option::None, syntax: &#syntax }
            });
            default = Some(parse);
            continue;
        }

        let name = attrs::name(&attrs, &variant.ident)?;
        attrs::unique(&name, &mut seen)?;

        specs.push(quote! {
            ::kagami::command::Subcommand { name: ::core::option::Option::Some(#name), syntax: &#syntax }
        });
        arms.push(quote! {
            ::core::option::Option::Some(#name) => {
                *__words = __next;
                #parse
            }
        });
        names.push(name.value());
    }

    let fallback = match default {
        Some(parse) => quote! {
            _ => {
                #parse
            }
        },
        None => {
            let message = format!("Expected one of: {}.", names.join(", "));

            quote! {
                ::core::option::Option::None => {
                    ::core::result::Result::Err(::kagami::command::ArgumentError::Missing("subcommand"))
                }
                ::core::option::Option::Some(_) => {
                    ::core::result::Result::Err(::kagami::command::ArgumentError::Invalid { name: "subcommand", message: #message })
                }
            }
        }
    };

    let syntax = quote!(::kagami::command::Syntax::Subcommands(&[#(#specs),*]));
    let parse = quote! {
        // Only a matching name is consumed: the default subcommand parses
        // from the same word.
        let mut __next = __words.clone();
        match __next.next() {
            #(#arms)*
            #fallback
        }
    };

    Ok((syntax, parse))
}

/// The syntax and parser of one subcommand's arguments: its named fields,
/// or the one type it holds.
fn subcommand(variant: &Variant, lifetime: &Lifetime) -> syn::Result<(Tokens, Tokens)> {
    let ident = &variant.ident;

    let Fields::Unnamed(fields) = &variant.fields else {
        return positional(&variant.fields, quote!(Self::#ident), lifetime);
    };

    if fields.unnamed.len() != 1 {
        return Err(syn::Error::new_spanned(fields, "a tuple subcommand holds exactly one CommandArgs type"));
    }

    let ty = &fields.unnamed[0].ty;
    let args = quote!(<#ty as ::kagami::command::CommandArgs<#lifetime>>);

    let syntax = quote!(#args::SYNTAX);
    let parse = quote!(#args::parse(__words).map(Self::#ident));

    Ok((syntax, parse))
}
