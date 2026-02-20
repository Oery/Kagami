use quote::quote;
use syn::{DeriveInput, Ident};

use crate::Origin;
use crate::utils::to_snake_case;

pub fn get_payload_impl(item: &DeriveInput, origin: &Origin) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let generics = &item.generics;

    let snake_name = to_snake_case(&name.to_string());
    let snake_orig = to_snake_case(&origin.to_string());
    let field = Ident::new(&format!("{snake_orig}_{snake_name}"), name.span());

    let (_, ty_generics, _) = generics.split_for_impl();

    let item_type = match generics.lifetimes().next().is_some() {
        true => quote! { #name<'b> },
        false => quote! { #name },
    };

    quote! {
        impl<'a> Payload<'a> for #name #ty_generics {
            type Item<'b> = #item_type;
            type Handler = Box<dyn for<'b> Fn(&mut Context<#item_type>) + Send + Sync + 'static>;

            fn register(em: &mut EventManager, f: Self::Handler) {
                em.packet_events.#field.push(f);
            }

            fn has_events(em: &EventManager) -> bool {
                !em.packet_events.#field.is_empty()
            }
        }
    }
}
