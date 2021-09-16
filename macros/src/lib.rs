#![forbid(unsafe_code)]

extern crate proc_macro;

mod attribute;
mod command;
mod effect;
mod event;
mod system;
mod task;

use attribute::impl_attribute;
use command::impl_command;
use effect::impl_effect;
use event::impl_event;
use system::impl_system;
use task::impl_task;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_attribute(attr, item)
}

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_command(attr, item)
}

#[proc_macro_attribute]
pub fn effect(attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_effect(attr, item)
}

#[proc_macro_attribute]
pub fn event(attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_event(attr, item)
}

#[proc_macro]
pub fn system(input: TokenStream) -> TokenStream {
    impl_system(input)
}

#[proc_macro]
pub fn task(input: TokenStream) -> TokenStream {
    impl_task(input)
}
