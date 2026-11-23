extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ReturnType, parse_macro_input};

/// Wraps the function to call the error macro if an error is returned
#[proc_macro_attribute]
pub fn err_wrap(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let ItemFn {
        sig,
        vis,
        block,
        attrs,
        ..
    } = input;

    match &sig.output {
        ReturnType::Default => {
            return syn::Error::new_spanned(sig.ident, "expected a Result return type")
                .to_compile_error()
                .into();
        }
        ReturnType::Type(_, ty) => {
            let is_result = matches!(
                ty.as_ref(),
                syn::Type::Path(type_path)
                    if type_path.path.segments.last()
                        .is_some_and(|segment| segment.ident == "Result")
            );

            if !is_result {
                return syn::Error::new_spanned(ty, "expected a Result return type")
                    .to_compile_error()
                    .into();
            }
        }
    }

    let name = sig.ident.to_string();

    let args = match sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(arg) => match arg.pat.as_ref() {
                syn::Pat::Ident(pat) => Ok(pat.ident.clone()),
                pat => Err(syn::Error::new_spanned(pat, "expected named parameters")),
            },
            receiver => Err(syn::Error::new_spanned(
                receiver,
                "receivers are not supported",
            )),
        })
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(args) => args,
        Err(error) => return error.to_compile_error().into(),
    };

    let inner_ident = quote::format_ident!("{}_inner", sig.ident);
    let mut inner_sig = sig.clone();
    inner_sig.ident = inner_ident.clone();
    let call = if sig.asyncness.is_some() {
        quote!(#inner_ident(#(#args),*).await)
    } else {
        quote!(#inner_ident(#(#args),*))
    };

    quote!(
        // Reapply all the other attributes on this function.
        #(#attrs)*
        // Reconstruct the function declaration
        #vis #sig {
            #inner_sig #block

            let __result = #call;

            if let Err(__e) = &__result {
                 log::error!("Error occurred in lib function {}: {}", #name, __e);
            }

            // Return the result (if any)
            __result
        }
    )
    .into()
}
