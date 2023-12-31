#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::unused_self)] // `self` is needed to change the behavior later
#![allow(clippy::missing_panics_doc)] // Temporary allow for `todo!`s
#![allow(clippy::new_without_default)] // Not very helpful as `new` is almost always cfged
#![cfg_attr(not(feature = "driver-api"), allow(dead_code))]
#![cfg_attr(marker, warn(marker::marker_lints::not_using_has_span_trait))]

pub static MARKER_API_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use interface::*;
pub use lint::*;

mod interface;
mod lint;
mod lint_pass;
mod private;

#[cfg(test)]
pub(crate) mod test;

pub mod ast;
pub mod common;
pub mod context;
pub mod diagnostic;
pub mod prelude;
pub mod sem;
pub mod span;

#[doc(hidden)]
pub mod ffi;

pub use context::MarkerContext;
pub use interface::LintPassInfoBuilder;

/// This struct blocks the construction of enum variants, similar to the `#[non_exhaustive]`
/// attribute.
///
/// Marker uses enums extensively, like [`ItemKind`][ast::ItemKind] and
/// [`ExprKind`](ast::ExprKind). There can be `*Kind` enums that wrap other
/// `*Kind` enums. In those cases, this struct is used, to block the user from
/// constructing the variant manually. This allows tools to handle the variants
/// confidently without additional verification. An example for this would be the
/// [`LitExprKind::UnaryOp`](ast::LitExprKind::UnaryOp) variant.
///
/// This basically acts like a `#[non_exhaustive]` attribute, with the difference
/// that it also works on tuple variants. Attaching `#[non_exhaustive]` to a tuple
/// variant would make it private, which we don't want.
///
/// As a normal user, you can just ignore this instance as it holds no relevant
/// information for linting.
#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone)]
pub struct CtorBlocker {
    /// `#[repr(C)]` requires a field, to make this a proper type. This is just
    /// the smallest one.
    _data: u8,
}

impl std::fmt::Debug for CtorBlocker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("..").finish()
    }
}

impl CtorBlocker {
    #[cfg_attr(feature = "driver-api", visibility::make(pub))]
    pub(crate) fn new() -> Self {
        Self { _data: 255 }
    }
}
