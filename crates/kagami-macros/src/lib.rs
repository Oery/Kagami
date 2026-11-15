//! Derives for `kagami::command`. They generate how commands parse and what
//! their syntax is, never what they do.

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod args;
mod attrs;
mod value;

/// A command: its arguments, as [`CommandArgs`] does, and its name.
#[proc_macro_derive(Command, attributes(command, arg))]
pub fn derive_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    args::expand_command(&input).unwrap_or_else(syn::Error::into_compile_error).into()
}

/// Positional arguments for a struct, subcommands for an enum.
#[proc_macro_derive(CommandArgs, attributes(command, arg))]
pub fn derive_command_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    args::expand_args(&input).unwrap_or_else(syn::Error::into_compile_error).into()
}

/// One word among the variants of a fieldless enum.
#[proc_macro_derive(CommandValue, attributes(value))]
pub fn derive_command_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    value::expand(&input).unwrap_or_else(syn::Error::into_compile_error).into()
}

#[cfg(test)]
#[path = "../tests/tests_derive.rs"]
mod tests;
