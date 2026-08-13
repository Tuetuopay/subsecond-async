//! Helper macros for [`subsecond_async`].

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Mark a function has hotpatchable.
///
/// Compatible with both sync and async functions.
///
/// # Usage
///
/// ```rust
/// # use subsecond_async::subsecond;
/// #[subsecond]
/// async fn tick() {
///     println!("Tick.");
/// }
/// ```
#[proc_macro_attribute]
pub fn subsecond(_args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let ItemFn { attrs, vis, modifiers, sig, block } = item;

    // Modifiers are not used as of writing and can't round-trip.
    if let Err(e) = modifiers.require_empty() {
        return e.to_compile_error().into();
    }

    let out = if sig.asyncness.is_some() {
        quote! {
            #(#attrs)* #vis #sig { ::subsecond_async::call(move || async move #block).await }
        }
    } else {
        quote! {
            #(#attrs)* #vis #sig { ::subsecond_async::call_sync(move || #block) }
        }
    };
    out.into()
}
