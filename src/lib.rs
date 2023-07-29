#![warn(
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]
#![doc = include_str!("../README.md")]

mod ninepatch;
pub use ninepatch::{NinePatch, NinePatchBuilder, NinePatchContent, Patch, Size};

mod plugin;
pub use plugin::*;
