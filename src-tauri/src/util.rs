use std::fmt::Display;
use strum::IntoEnumIterator;

pub fn enum_to_vec<E : IntoEnumIterator + Display>() -> Vec<String> {
    E::iter()
        .map(|unit_type| unit_type.to_string())
        .collect::<Vec<String>>()
}