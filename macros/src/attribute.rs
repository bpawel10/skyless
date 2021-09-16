use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse, ItemStruct};

pub fn impl_attribute(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse::<ItemStruct>(item).unwrap();
    let span = Span::call_site();
    let attribute_type = &item.ident;
    let attribute_name = Ident::new(
        &attribute_type
            .to_string()
            .from_case(Case::Pascal)
            .to_case(Case::Snake),
        span,
    );
    let attribute_name_string = attribute_name.to_string();
    let attribute_methods = quote! {
        pub fn #attribute_name(&self) -> Option<&#attribute_type> {
            match self.attributes.get(#attribute_name_string) {
                Some(attribute) => {
                    attribute.as_any().downcast_ref::<#attribute_type>()
                },
                _ => None,
            }
        }
    };
    let gen = quote! {
        #[derive(Debug, Clone)]
        #item

        impl Attribute for #attribute_type {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn as_name(&self) -> &str {
                #attribute_name_string
            }
        }

        impl GameAttributes {
            #attribute_methods
        }

        impl Tile {
            #attribute_methods
        }

        impl Entity {
            #attribute_methods
        }
    };
    gen.into()
}
