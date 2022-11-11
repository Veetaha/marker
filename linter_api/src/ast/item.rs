use std::{fmt::Debug, marker::PhantomData};

use super::{ty::TyKind, Abi, ItemId, Safety, Span, SymbolId};

// Item implementations
mod extern_crate_item;
pub use self::extern_crate_item::ExternCrateItem;
mod mod_item;
pub use mod_item::ModItem;
mod static_item;
pub use self::static_item::StaticItem;
mod use_decl_item;
pub use self::use_decl_item::UseDeclItem;
mod const_item;
pub use self::const_item::ConstItem;
mod fn_item;
pub use fn_item::*;
mod ty_alias_item;
pub use ty_alias_item::*;
mod adt_item;
pub use adt_item::*;
mod trait_item;
pub use trait_item::*;

pub trait ItemData<'ast>: Debug {
    /// Returns the [`ItemId`] of this item. This is a unique identifier used for comparison
    /// and to request items from the [`AstContext`][`crate::context::AstContext`].
    fn id(&self) -> ItemId;

    /// The [`Span`] of the entire item. This span should be used for general item related
    /// diagnostics.
    fn span(&self) -> &Span<'ast>;

    /// The visibility of this item.
    fn visibility(&self) -> &Visibility<'ast>;

    /// This function can return `None` if the item was generated and has no real name
    fn name(&self) -> Option<String>;

    /// This returns this [`ItemData`] instance as a [`ItemKind`]. This can be useful for
    /// functions that take [`ItemKind`] as a parameter. For general function calls it's better
    /// to call them directoly on the item, instead of converting it to a [`ItemKind`] first.
    fn as_item(&'ast self) -> ItemKind<'ast>;

    fn attrs(&self); // FIXME: Add return type: -> &'ast [&'ast dyn Attribute<'ast>];
}

#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum ItemKind<'ast> {
    Mod(&'ast ModItem<'ast>),
    ExternCrate(&'ast ExternCrateItem<'ast>),
    UseDecl(&'ast UseDeclItem<'ast>),
    Static(&'ast StaticItem<'ast>),
    Const(&'ast ConstItem<'ast>),
    Fn(&'ast FnItem<'ast>),
    TyAlias(&'ast TyAliasItem<'ast>),
    Struct(&'ast StructItem<'ast>),
    Enum(&'ast EnumItem<'ast>),
    Union(&'ast UnionItem<'ast>),

    Trait(&'ast TraitItem<'ast>),
    Impl(&'ast dyn ImplItem<'ast>),
    ExternBlock(&'ast dyn ExternBlockItem<'ast>),
}

impl<'ast> ItemKind<'ast> {
    impl_item_type_fn!(ItemKind: id() -> ItemId);
    impl_item_type_fn!(ItemKind: span() -> &Span<'ast>);
    impl_item_type_fn!(ItemKind: visibility() -> &Visibility<'ast>);
    impl_item_type_fn!(ItemKind: name() -> Option<String>);
    impl_item_type_fn!(ItemKind: attrs() -> ());
}

#[non_exhaustive]
#[derive(Debug)]
pub enum AssocItemKind<'ast> {
    TyAlias(&'ast TyAliasItem<'ast>),
    Const(&'ast ConstItem<'ast>),
    Fn(&'ast FnItem<'ast>),
}

impl<'ast> AssocItemKind<'ast> {
    impl_item_type_fn!(AssocItemKind: id() -> ItemId);
    impl_item_type_fn!(AssocItemKind: span() -> &Span<'ast>);
    impl_item_type_fn!(AssocItemKind: visibility() -> &Visibility<'ast>);
    impl_item_type_fn!(AssocItemKind: name() -> Option<String>);
    impl_item_type_fn!(AssocItemKind: attrs() -> ());
    impl_item_type_fn!(AssocItemKind: as_item() -> ItemKind<'ast>);
}

/// Until [trait upcasting](https://github.com/rust-lang/rust/issues/65991) has been implemented
/// and stabalized we need this to call [`ItemData`] functions for [`ItemKind`].
macro_rules! impl_item_type_fn {
    (ItemKind: $method:ident () -> $return_ty:ty) => {
        impl_item_type_fn!((ItemKind) $method() -> $return_ty,
            Mod, ExternCrate, UseDecl, Static, Const, Fn,
            TyAlias, Struct, Enum, Union, Trait, Impl, ExternBlock
        );
    };
    (AssocItemKind: $method:ident () -> $return_ty:ty) => {
        impl_item_type_fn!((AssocItemKind) $method() -> $return_ty,
            TyAlias, Const, Fn
        );
    };

    (($self:ident) $method:ident () -> $return_ty:ty $(, $item:ident)+) => {
        pub fn $method(&self) -> $return_ty {
            match self {
                $($self::$item(data) => data.$method(),)*
            }
        }
    };
}

use impl_item_type_fn;

#[repr(C)]
#[derive(Debug)]
#[cfg_attr(feature = "driver-api", visibility::make(pub))]
struct CommonItemData<'ast> {
    id: ItemId,
    vis: Visibility<'ast>,
    name: SymbolId,
}

macro_rules! impl_item_data {
    ($self_name:ident, $enum_name:ident) => {
        impl<'ast> super::ItemData<'ast> for $self_name<'ast> {
            fn id(&self) -> crate::ast::item::ItemId {
                self.data.id
            }

            fn span(&self) -> &crate::ast::Span<'ast> {
                $crate::context::with_cx(self, |cx| cx.get_span(self.data.id))
            }

            fn visibility(&self) -> &crate::ast::item::Visibility<'ast> {
                &self.data.vis
            }

            fn name(&self) -> Option<String> {
                Some($crate::context::with_cx(self, |cx| cx.symbol_str(self.data.name)))
            }

            fn as_item(&'ast self) -> crate::ast::item::ItemKind<'ast> {
                $crate::ast::item::ItemKind::$enum_name(self)
            }

            fn attrs(&self) {
                todo!()
            }
        }
    };
}

use impl_item_data;

#[cfg(feature = "driver-api")]
impl<'ast> CommonItemData<'ast> {
    pub fn new(id: ItemId, vis: Visibility<'ast>, name: SymbolId) -> Self {
        Self { id, vis, name }
    }
}

/// FIXME: Add function as  discussed in <https://github.com/rust-linting/design/issues/22>
/// this will require new driver callback functions
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Visibility<'ast> {
    _lifetime: PhantomData<&'ast ()>,
    _item_id: ItemId,
}

#[cfg(feature = "driver-api")]
impl<'ast> Visibility<'ast> {
    pub fn new(item_id: ItemId) -> Self {
        Self {
            _lifetime: PhantomData,
            _item_id: item_id,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Items based on traits
///////////////////////////////////////////////////////////////////////////////
/// An anonymous constant.
pub trait AnonConst<'ast>: Debug {
    fn get_ty(&self);

    // FIXME: This should return a expression once they are implemented, it would
    // probably be good to have an additional `get_value_lit` that returns a literal,
    // if the value can be represented as one.
    fn get_value(&self);
}

pub trait ImplItem<'ast>: ItemData<'ast> {
    fn get_inner_attrs(&self); // FIXME: Add return type -> [&dyn Attribute<'ast>];

    fn get_safety(&self) -> Safety;

    fn get_polarity(&self) -> ImplPolarity;

    /// This will return `Some` if this is a trait implementation, otherwiese `None`.
    fn get_trait(&self) -> Option<&TyKind<'ast>>;

    fn get_ty(&self) -> &TyKind<'ast>;

    fn get_generics(&self);

    fn get_assoc_items(&self) -> &[AssocItemKind<'ast>];
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ImplPolarity {
    Positive,
    /// A negative implementation like:
    /// ```ignore
    /// unsafe impl !Send for ImplPolarity;
    /// //          ^
    /// ```
    Negative,
}

pub trait ExternBlockItem<'ast>: ItemData<'ast> {
    fn get_inner_attrs(&self); // FIXME: Add return type -> [&dyn Attribute<'ast>];

    fn get_safety(&self) -> Safety;

    fn get_abi(&self) -> Abi;

    fn get_external_items(&self) -> ExternalItems<'ast>;
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ExternalItems<'ast> {
    Static(&'ast StaticItem<'ast>),
    Function(&'ast FnItem<'ast>),
}
