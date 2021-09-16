use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse, Ident, ItemStruct};

pub fn impl_event(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse::<ItemStruct>(item).unwrap();
    let span = Span::call_site();
    let event_type = &item.ident;
    let event_name = Ident::new(
        &event_type
            .to_string()
            .from_case(Case::Pascal)
            .to_case(Case::Snake),
        span,
    );
    let event_name_string = event_name.to_string();
    let gen = quote! {
        #[derive(Debug)]
        #item

        impl Event for #event_type {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_name(&self) -> &str {
                #event_name_string
            }
        }
    };
    gen.into()
}
