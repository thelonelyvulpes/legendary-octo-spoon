use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn};


#[proc_macro_attribute]
pub fn time_fn(data_expr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(item as ItemFn);

    if !cfg!(feature = "profiling") {
        let original = quote! {
            #(#attrs)*
            #vis #sig #block
        };

        return TokenStream::from(original)
    };
    let v = sig.ident.to_string();
    let expanded = if data_expr.is_empty() {
        quote! {
            #(#attrs)*
            #vis #sig {
                unsafe {
                    let __measure_time_start_instant = core::arch::x86_64::_rdtsc();
                    let ret = #block;
                    let __measure_time_end_instant = core::arch::x86_64::_rdtsc();
                    use crate::{PROFILE_RECORDS, ProfPoint};
                    PROFILE_RECORDS.values.push(ProfPoint::Open(__measure_time_start_instant, #v));
                    PROFILE_RECORDS.values.push(ProfPoint::Close(__measure_time_end_instant));
                    ret
                }
            }
        }
    } else {
        let data = parse_macro_input!(data_expr as Expr);
        quote! {
            #(#attrs)*
            #vis #sig {
                unsafe {
                    let __measure_time_start_instant = core::arch::x86_64::_rdtsc();
                    let result = #block;
                    let __measure_time_end_instant = core::arch::x86_64::_rdtsc();
                    use crate::{PROFILE_RECORDS, ProfPoint};
                    PROFILE_RECORDS.values.push(ProfPoint::Open(__measure_time_start_instant, #v));
                    PROFILE_RECORDS.values.push(ProfPoint::Data(#data));
                    PROFILE_RECORDS.values.push(ProfPoint::Close(__measure_time_end_instant));
                    result
                }
            }
        }
    };

    TokenStream::from(expanded)
}
