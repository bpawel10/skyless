use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn impl_system(input: TokenStream) -> TokenStream {
    let input_parsed: TokenStream2 = input.into();
    let gen = quote! {
        pub async fn system(
            game: Arc<Mutex<Game>>,
            tasker: Sender<TaskType>,
        ) -> IoResult<()> {
            #input_parsed
            Ok(())
        }
    };
    gen.into()
}
