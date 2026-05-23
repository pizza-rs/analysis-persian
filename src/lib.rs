#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod arabic_normalization;
mod normalization;
mod register;
mod stem;
mod stop;

pub use arabic_normalization::ArabicNormalizationFilter;
pub use normalization::PersianNormalizationFilter;
pub use register::register_all;
pub use stem::PersianStemFilter;
pub use stop::PersianStopFilter;
