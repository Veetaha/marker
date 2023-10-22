use marker_api::prelude::*;

marker_api::declare_lint! {
    /// # What it does
    /// Uses `MarkerContext::ast().item()` to get the AST node for an item.
    /// [Original bug report](https://github.com/rust-marker/marker/issues/301).
    ISSUE_301,
    Deny,
}

pub(crate) fn check_item<'ast>(cx: &'ast MarkerContext<'ast>, item: ast::ItemKind<'ast>) {
    let ItemKind::Impl(impl_) = item else {
        return;
    };

    if item.span().is_from_expansion() {
        return;
    }

    let Some(trait_ref) = impl_.trait_ref() else {
        return;
    };

    cx.ast().item(trait_ref.trait_id());
}
