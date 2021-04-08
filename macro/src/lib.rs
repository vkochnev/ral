use proc_macro::TokenStream;

mod register;

#[proc_macro]
pub fn register(item: TokenStream) -> TokenStream {
    crate::register::register(item)
}

#[proc_macro]
pub fn register_uses(item: TokenStream) -> TokenStream {
    crate::register::register_uses(item)
}

#[proc_macro]
pub fn register_definition(item: TokenStream) -> TokenStream {
    crate::register::register_definition(item)
}
