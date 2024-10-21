//! This module contains utility methods for macros.

use syn::{Expr, Ident, ItemEnum, Lit};

pub fn preprocess_string_enum(item_enum: &mut ItemEnum) -> Vec<(Ident, String)> {
    item_enum
        .variants
        .iter_mut()
        .map(|field| {
            let key = field.ident.clone();

            let value = match &field.discriminant {
                Some((_, Expr::Lit(elit))) => match &elit.lit {
                    Lit::Str(lit_str) => lit_str.value(),
                    _ => key.to_string(),
                },
                // Some((eq, expr)) => syn::Error::new(expr.span(), "expected a string literal").into_compile_error(),
                //     .to_compile_error()
                //     .spanned(eq.span())
                //     .to_compile_error(),
                // None => key.to_string(),
                _ => key.to_string(),
            };

            field.discriminant = None;

            (key, value)
        })
        .collect::<Vec<_>>()
}
