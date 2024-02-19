pub use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, ItemFn};

pub fn impl_effect(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_into: TokenStream2 = item.clone().into();
    let event_type = syn::parse::<Ident>(attr).unwrap();
    let event_name_string = event_type
        .to_string()
        .from_case(Case::Pascal)
        .to_case(Case::Snake);
    let event_fn = syn::parse::<ItemFn>(item).unwrap();
    let event_fn_name = event_fn.sig.ident;
    let gen = quote! {
        #item_into
        let listener = Box::new(#event_fn_name);
        {
            let mut game = game.lock().unwrap();
            let mut listeners = game.listeners.write().unwrap();
            match listeners.get_mut(#event_name_string) {
                Some(event_listeners) => { event_listeners.push(listener); },
                None => { listeners.insert(#event_name_string.to_string(), vec![listener]); },
            };
        }
    };
    gen.into()
}
