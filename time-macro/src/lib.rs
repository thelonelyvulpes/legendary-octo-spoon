use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn, Lit, MetaNameValue};

#[proc_macro_attribute]
pub fn time_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as MetaNameValue);
    let v = if let Expr::Lit(base) = attrs.value {
        match base.lit {
            Lit::Int(x) => x.base10_parse::<u32>().unwrap(),
            _ => 0,
        }
    } else {
        0
    };
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(item as ItemFn);

    let start = quote!(__measure_time_start_instant);
    let end = quote!(__measure_time_start_instant);

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            unsafe {
                let #start = core::arch::x86_64::_rdtsc();
                let ret = #block;
                let #end = core::arch::x86_64::_rdtsc();
                use crate::{PROFILE_RECORDS};
                PROFILE_RECORDS.values.push((#v, #end-#start));
                ret
            }
        }
    };

    TokenStream::from(expanded)
}
