use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn time_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn { attrs, vis, sig, block }
        = parse_macro_input!(item as ItemFn);

    let fn_name = sig.ident.to_string();
    let start = quote!(__measure_time_start_instant);
    let end = quote!(__measure_time_start_instant);
    

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            unsafe {
                let #start = core::arch::x86_64::_rdtsc();
                let ret = #block;
                let #end = core::arch::x86_64::_rdtsc();
                std::println!("{} took: {:#?}", #fn_name, #end - #start);
                ret
            }
        }
    };

    TokenStream::from(expanded)
}