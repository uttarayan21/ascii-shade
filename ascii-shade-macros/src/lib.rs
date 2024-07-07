use internal::str_to_const_chars;
use proc_macro::*;
use quote::ToTokens as _;

#[proc_macro]
pub fn shader_map(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as internal::InputArgs);
    let output = str_to_const_chars(input);
    quote::quote! {
        ShaderMap {
            map: Cow::Borrowed(#output.as_slice())
        }

    }
    .into()
}

mod internal {
    use syn::{parse_quote, punctuated::Punctuated, token::Comma, Expr};
    type Result<T, E = syn::Error> = std::result::Result<T, E>;

    pub fn str_to_const_chars(input: InputArgs) -> syn::ExprArray {
        let chars: Punctuated<Expr, Comma> = input
            .input
            .value()
            .chars()
            .map(|c| -> Expr {
                parse_quote! {
                    #c
                }
            })
            .collect();
        syn::ExprArray {
            attrs: Vec::new(),
            bracket_token: Default::default(),
            elems: chars,
        }
    }

    pub struct InputArgs {
        pub input: syn::LitStr,
    }

    impl syn::parse::Parse for InputArgs {
        fn parse(input: syn::parse::ParseStream) -> Result<Self> {
            Ok(Self {
                input: input.parse()?,
            })
        }
    }
}
