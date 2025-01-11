extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn __applescript(input: TokenStream) -> TokenStream {
    let input = input.to_string().replace("\n", " ");
    let string_literals = input.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| format!("{}", s));
    let output = quote! {
        run_applescript(&[#(#string_literals),*])
    };
    
    TokenStream::from(output)
}