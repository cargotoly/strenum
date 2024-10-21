//! This module contains the procedural macro implementation
//! for the `StringEnum` attribute.

pub(crate) mod util;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

/// This procedural macro generates JS 'string-like' enums,
/// which support from and to string conversions on all
/// members.
///
/// ```
/// use strenum::*;
/// use std::str::FromStr;
///
/// #[derive(Debug, PartialEq)]
/// #[StringEnum]
/// pub enum Method {
///     Get = "Hi",
///     Post,
/// }
///
/// assert_eq!(Method::Get.to_string(), "Hi".to_string());
/// assert_eq!(Method::Post.to_string(), "Post".to_string());
/// assert_eq!(Method::from_str("Hi").unwrap(), Method::Get);
/// assert_eq!(Method::from_str("Post").unwrap(), Method::Post);
/// assert!(Method::from_str("other").is_err());
/// ```
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn StringEnum(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // TODO implement potential usage for attr
    // #[StringEnum = KebabCase]

    // TODO implement flags for case sensitivity
    // #[StringEnum(no_case)]

    // https://docs.rs/syn/latest/src/syn/item.rs.html#2072-2093
    // https://docs.rs/syn/latest/src/syn/derive.rs.html#184-198
    // https://docs.rs/syn/latest/src/syn/data.rs.html#256-295

    let mut item_enum = parse_macro_input!(input as ItemEnum);

    // A list of variants. We expect the length of the enum
    // members to be in ascending order, therefore for parsing,
    // the list has to be reversed to descending order.
    let mut variants = util::preprocess(&mut item_enum);
    variants.reverse();

    // The name of the enum. This is used to reference the enum
    // in the generated code.
    let name = item_enum.ident.clone();

    // A vector of match arm tokens, which are used to match
    // the input as a string prefix to the corresponding enum
    // variant.
    //
    // Example:
    // ```rs
    // _ if input.starts_with("Get") => Some((&input[3..], Self::Get)),
    // ```
    let starts_with_arms = variants
        .iter()
        .map(|(key, value)| {
            let len = value.len();
            quote![_ if input.starts_with(#value) => Some((&input[#len..], Self::#key)),]
        })
        .collect::<Vec<_>>();

    // A vector of match arm tokens, which are used to match
    // the input as a string to the corresponding enum variant.
    //
    // Example:
    // ```rs
    // "Get" => Ok(Self::Get),
    // ```
    let from_str_arms = variants
        .iter()
        .map(|(key, value)| quote![#value => Ok(Self::#key),])
        .collect::<Vec<_>>();

    // A vector of match arm tokens, which are used to match
    // the enum variant to the corresponding string.
    //
    // Example:
    // ```rs
    // Self::Get => "Get",
    // ```
    let to_str_arms = variants
        .iter()
        .map(|(key, value)| quote![Self::#key => #value,])
        .collect::<Vec<_>>();

    // Find the longest variant in the enum. This is used to
    // generated constant values for the `MAX_VARIANT_LEN`
    let (max_variant, max_variant_len) = {
        let max_variant = variants
            .iter()
            .map(|(_, value)| value)
            .fold(
                String::new(),
                |a, b| if a.len() > b.len() { a } else { b.clone() },
            );
        let len = max_variant.len();
        (max_variant, len)
    };

    // A document string for the generated MAX_VARIANT_LEN constant.
    let max_variant_comment = format!(
        "The string size of the longest enum member string.\n\n```rs\n\"{}\" // len = {} = {:#X}\n```\n\n",
        max_variant, max_variant_len, max_variant_len
    );

    quote! {
        #[non_exhaustive]
        #item_enum

        impl #name {
            #[doc = #max_variant_comment]
            /// The length of longest the string slice. This
            /// can be used in a pattern matcher to guarantee
            /// a mismatch with current enum string.
            pub const MAX_VARIANT_LEN: usize = #max_variant_len;

            /// Returns a nom-style combinator for parsing the
            /// enum from a string input.
            pub fn combinator() -> impl Fn(&str) -> Option<(&str, Self)> {
                move |input: &str| {
                    match true {
                        #(#starts_with_arms)*
                        _ => None,
                    }
                }
            }

            /// Returns the length of the string representation.
            pub fn len(&self) -> usize {
                self.to_string().len()
            }
        }

        impl std::str::FromStr for #name {
            type Err = ();

            fn from_str(method: &str) -> Result<Self, Self::Err> {
                match method {
                    #(#from_str_arms)*
                    _ => Err(())
                }
            }
        }

        // ToString implemented: https://stackoverflow.com/a/27770058/16002144
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match self {
                    #(#to_str_arms)*
                    _ => unreachable!()
                })
            }
        }

        impl std::convert::Into<String> for #name {
            fn into(self) -> String {
                use std::string::ToString;
                self.to_string()
            }
        }
    }
    .into()
}
