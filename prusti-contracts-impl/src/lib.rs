#![no_std]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn requires(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn invariant(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn ensures(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn after_expiry(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn assert_on_expiry(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn pure(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn trusted(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro]
pub fn body_invariant(_tokens: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn prusti_assert(_tokens: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn prusti_assume(_tokens: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn refine_trait_spec(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn extern_spec(_attr: TokenStream, _tokens: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn predicate(_tokens: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, _tokens: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn ghost_constraint(_attr: TokenStream, _tokens: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn ghost(tokens: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = tokens.into();
    quote::quote! {{
        use prusti_contracts::*;

        trait GhostGetter<T> {
            fn get(self) -> T;
            fn set(self, t: T);
        }
        impl<T> GhostGetter<T> for Ghost<T> {
            fn get(self) -> T {
                panic!();
            }
            fn set(self, t: T) {
                panic!()
            }
        }

        let ghost_closure = || {
            #tokens
        };
        unsafe{Ghost::unsafe_new(&ghost_closure)}
    }}
    .into()
}
