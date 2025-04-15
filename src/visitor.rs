use std::{marker::PhantomData, mem::replace};

use syn::{
    spanned::Spanned,
    token::Eq,
    visit_mut::{visit_expr_mut, VisitMut},
    BinOp, Expr, ExprAssign, ExprBinary, ExprUnary,
};

use crate::{
    block::MathBlock,
    expr::{empty_expr_binary, empty_expr_unary, method_call},
};

pub struct MathBlockVisitor<B>(pub PhantomData<B>);

impl<B> MathBlockVisitor<B> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<B: MathBlock> VisitMut for MathBlockVisitor<B> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        match node {
            Expr::Binary(bin) => 'handled: {
                let Some(method_ident) = B::method_ident_bin(bin.op) else {
                    break 'handled;
                };

                let ExprBinary {
                    attrs,
                    left,
                    op,
                    right,
                } = replace(bin, empty_expr_binary());

                *node = if is_assign(op) {
                    let call = method_call(
                        method_ident,
                        left.clone(),
                        Some(right),
                        op.span(),
                        Vec::new(),
                    );

                    let expr = B::finalize_bin(Expr::MethodCall(call), op);

                    Expr::Assign(ExprAssign {
                        attrs,
                        left,
                        eq_token: Eq(op.span()),
                        right: Box::new(expr),
                    })
                } else {
                    let call =
                        method_call(method_ident, left.clone(), Some(right), op.span(), attrs);

                    B::finalize_bin(Expr::MethodCall(call), op)
                };
            }
            Expr::Unary(un) => 'handled: {
                let Some(method_ident) = B::method_ident_un(un.op) else {
                    break 'handled;
                };

                let ExprUnary { attrs, op, expr } = replace(un, empty_expr_unary());

                let call = method_call(method_ident, expr, None, op.span(), attrs);

                *node = B::finalize_un(Expr::MethodCall(call), op);
            }
            _ => {}
        }

        visit_expr_mut(self, node);
    }
}

const fn is_assign(op: BinOp) -> bool {
    match op {
        BinOp::Add(_)
        | BinOp::Sub(_)
        | BinOp::Mul(_)
        | BinOp::Div(_)
        | BinOp::Rem(_)
        | BinOp::And(_)
        | BinOp::Or(_)
        | BinOp::BitXor(_)
        | BinOp::BitAnd(_)
        | BinOp::BitOr(_)
        | BinOp::Shl(_)
        | BinOp::Shr(_)
        | BinOp::Eq(_)
        | BinOp::Lt(_)
        | BinOp::Le(_)
        | BinOp::Ne(_)
        | BinOp::Ge(_)
        | BinOp::Gt(_) => false,
        BinOp::AddAssign(_)
        | BinOp::SubAssign(_)
        | BinOp::MulAssign(_)
        | BinOp::DivAssign(_)
        | BinOp::RemAssign(_)
        | BinOp::BitXorAssign(_)
        | BinOp::BitAndAssign(_)
        | BinOp::BitOrAssign(_)
        | BinOp::ShlAssign(_)
        | BinOp::ShrAssign(_) => true,
        _ => false,
    }
}
