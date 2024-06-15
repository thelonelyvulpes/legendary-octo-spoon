use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn time_fn(_: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(item as ItemFn);

    let v = sig.ident.to_string();

    let start = quote!(__measure_time_start_instant);
    let end = quote!(__measure_time_start_instant);

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            unsafe {
                use crate::{PROFILE_RECORDS, ProfPoint};
                let #start = core::arch::x86_64::_rdtsc();
                PROFILE_RECORDS.values.push(ProfPoint::Open(#start, #v));
                let ret = #block;
                let #end = core::arch::x86_64::_rdtsc();
                PROFILE_RECORDS.values.push(ProfPoint::Close(#end));
                ret
            }
        }
    };

    TokenStream::from(expanded)
}
