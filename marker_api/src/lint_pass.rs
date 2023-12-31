use crate::interface::LintPassInfo;
use crate::{ast, MarkerContext};

/// A [`LintPass`] visits every node like a `Visitor`. The difference is that a
/// [`LintPass`] provides some additional information about the implemented lints.
/// The adapter will walk through the entire AST once and give each node to the
/// registered [`LintPass`]es.
pub trait LintPass {
    fn check_crate<'ast>(&mut self, _cx: &'ast MarkerContext<'ast>, _krate: &'ast ast::Crate<'ast>) {}
    fn check_item<'ast>(&mut self, _cx: &'ast MarkerContext<'ast>, _item: ast::ItemKind<'ast>) {}
    fn check_field<'ast>(&mut self, _cx: &'ast MarkerContext<'ast>, _field: &'ast ast::ItemField<'ast>) {}
    fn check_variant<'ast>(&mut self, _cx: &'ast MarkerContext<'ast>, _variant: &'ast ast::EnumVariant<'ast>) {}
    fn check_body<'ast>(&mut self, _cx: &'ast MarkerContext<'ast>, _body: &'ast ast::Body<'ast>) {}
    fn check_stmt<'ast>(&mut self, _cx: &'ast MarkerContext<'ast>, _stmt: ast::StmtKind<'ast>) {}
    fn check_expr<'ast>(&mut self, _cx: &'ast MarkerContext<'ast>, _expr: ast::ExprKind<'ast>) {}
}

#[doc(hidden)]
pub trait LintPassBase<'ast> {
    fn new(cx: MarkerContext<'ast>) -> Self
    where
        Self: Sized;

    fn info(&self) -> LintPassInfo;
}
