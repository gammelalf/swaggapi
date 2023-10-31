mod operation;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn operation(args: TokenStream, input: TokenStream) -> TokenStream {
    operation::operation(args.into(), input.into()).into()
}
