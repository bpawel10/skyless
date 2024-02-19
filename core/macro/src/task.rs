use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn impl_task(input: TokenStream) -> TokenStream {
    let content = TokenStream2::from(input);
    let gen = quote! {
        // TODO: handle error when send fails
        tasker.clone().send(Box::pin(stream! { #content })).await;
    };
    gen.into()
}
