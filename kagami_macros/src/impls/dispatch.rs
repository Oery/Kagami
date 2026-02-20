use quote::quote;
use syn::{Generics, Ident};

use crate::Origin;
use crate::utils::to_snake_case;

pub fn get_dispatch_impl(name: &Ident, origin: &Origin, generics: &Generics) -> proc_macro2::TokenStream {
    let snake_name = to_snake_case(&name.to_string());
    let snake_orig = to_snake_case(&origin.to_string());
    let field = Ident::new(&format!("{snake_orig}_{snake_name}"), name.span());

    let (_, ty_generics, _) = generics.split_for_impl();

    let new_pctx = match origin {
        Origin::Client => quote! { Context::new(self, &mut ctx.src, &mut ctx.dst, &ctx.state) },
        Origin::Server => quote! { Context::new(self, &mut ctx.dst, &mut ctx.src, &ctx.state) },
    };

    quote! {
        impl<'a> Dispatch<'a> for #name #ty_generics {
            fn dispatch(self, ctx: &mut crate::context::ProxyContext) -> Option<Self> {
                let mut pctx = #new_pctx;

                for event in &ctx.proxy.events.packet_events.#field {
                    event(&mut pctx);
                }

                match pctx.cancel {
                    true => None,
                    false => Some(pctx.payload),
                }
            }
        }
    }
}
