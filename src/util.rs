//! This module contains utility methods for macros.

use syn::{Expr, Ident, ItemEnum, Lit};

pub fn preprocess(item_enum: &mut ItemEnum) -> Vec<(Ident, String)> {
    item_enum
        .variants
        .iter_mut()
        .map(|field| {
            let key = field.ident.clone();

            let value = match &field.discriminant {
                Some((_, expr)) => match expr {
                    Expr::Lit(elit) => match &elit.lit {
                        Lit::Str(lit_str) => lit_str.value(),
                        _ => key.to_string(),
                    },
                    _ => key.to_string(),
                },
                None => key.to_string(),
            };

            field.discriminant = None;

            (key, value)
        })
        .collect::<Vec<_>>()
}
