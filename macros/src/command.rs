use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, ItemStruct};

pub fn impl_command(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse::<ItemStruct>(item).unwrap();
    let command_type = &item.ident;
    let gen = quote! {
        #[derive(Debug)]
        #item

        impl Command for #command_type {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
                Box::new(self)
            }
        }
    };
    gen.into()
}
