use crate::{
    ast::ty::TyKind,
    ffi::{FfiOption, FfiSlice},
};

use super::{Abi, SpanId, SymbolId};

/// This trait provides informations about callable items and types. Some
/// properties might not be available for every callable object. In those
/// cases the default value will be returned.
pub trait Callable<'ast> {
    /// Returns `true`, if this callable is `const`.
    ///
    /// Defaults to `false` if unspecified.
    fn is_const(&self) -> bool;

    /// Returns `true`, if this callable is `async`.
    ///
    /// Defaults to `false` if unspecified.
    fn is_async(&self) -> bool;

    /// Returns `true`, if this callable is marked as `unsafe`.
    ///
    /// Defaults to `false` if unspecified. Extern functions will
    /// also return `false` by default, even if they require `unsafe`
    /// by default.
    fn is_unsafe(&self) -> bool;

    /// Returns `true`, if this callable is marked as extern.
    ///
    /// Defaults to `false` if unspecified.
    fn is_extern(&self) -> bool;

    /// Returns the [`Abi`] of the callable, if specified.
    fn abi(&self) -> Option<Abi>;

    /// Returns `true`, if this callable has a specified `self` argument. The
    /// type of `self` can be retrieved from the first element of
    /// [`Callable::params()`].
    fn has_self(&self) -> bool;

    /// Returns the parameters, that this callable accepts. The `self` argument
    /// of methods, will be the first element of this slice. Use
    /// [`Callable::has_self`] to determine if the first argument is `self`.
    fn params(&self) -> &[&Parameter<'ast>];

    /// Returns the return type, if specified.
    fn return_ty(&self) -> Option<&TyKind<'ast>>;
}

#[repr(C)]
#[derive(Debug)]
pub struct Parameter<'ast> {
    name: FfiOption<SymbolId>,
    ty: FfiOption<TyKind<'ast>>,
    span: FfiOption<SpanId>,
}

impl<'ast> Parameter<'ast> {
    // Function items actually use patterns and not names. Patterns are not yet
    // implemented though. A pattern should be good enough for now.
    pub fn name(&self) -> Option<SymbolId> {
        self.name.get().copied()
    }

    pub fn ty(&self) -> Option<TyKind<'ast>> {
        self.ty.get().copied()
    }

    pub fn span(&self) -> Option<SpanId> {
        self.span.get().copied()
    }
}

#[repr(C)]
#[derive(Debug)]
#[cfg_attr(feature = "driver-api", visibility::make(pub))]
pub(crate) struct CallableData<'ast> {
    pub(crate) is_const: bool,
    pub(crate) is_async: bool,
    pub(crate) is_unsafe: bool,
    pub(crate) is_extern: bool,
    pub(crate) abi: FfiOption<Abi>,
    pub(crate) has_self: bool,
    pub(crate) params: FfiSlice<'ast, &'ast Parameter<'ast>>,
    pub(crate) return_ty: FfiOption<TyKind<'ast>>,
}

macro_rules! impl_callable_trait {
    ($self_ty:ty) => {
        impl<'ast> $crate::ast::common::Callable<'ast> for $self_ty {
            fn is_const(&self) -> bool {
                self.callable_data.is_const
            }
            fn is_async(&self) -> bool {
                self.callable_data.is_async
            }
            fn is_unsafe(&self) -> bool {
                self.callable_data.is_unsafe
            }
            fn is_extern(&self) -> bool {
                self.callable_data.is_extern
            }
            fn abi(&self) -> Option<$crate::ast::common::Abi> {
                self.callable_data.abi.get().copied()
            }
            fn has_self(&self) -> bool {
                self.callable_data.has_self
            }
            fn params(&self) -> &[&$crate::ast::common::Parameter<'ast>] {
                self.callable_data.params.get()
            }
            fn return_ty(&self) -> Option<&$crate::ast::ty::TyKind<'ast>> {
                self.callable_data.return_ty.get()
            }
        }
    };
}
pub(crate) use impl_callable_trait;
