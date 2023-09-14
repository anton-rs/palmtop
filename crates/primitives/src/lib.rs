#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/refcell/palmtop/main/extra/palmtop.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/refcell/palmtop/issues/"
)]
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! Palmtop Primitives
//!
//! This crate contains the core primitives for palmtop.

/// Preimage Oracle Primitives.
pub mod preimage;
pub use preimage::{Preimage, PreimageGetter, PreimageKey};
