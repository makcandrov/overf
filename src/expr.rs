use proc_macro2::Span;
use syn::{
    Attribute, BinOp, Expr, ExprBinary, ExprMethodCall, ExprUnary, Ident, UnOp,
    punctuated::Punctuated,
    token::{Dot, Paren},
};

pub fn method_call(
    ident: &str,
    left: Box<Expr>,
    right: Option<Box<Expr>>,
    span: Span,
    attrs: Vec<Attribute>,
) -> ExprMethodCall {
    let mut args = Punctuated::new();
    if let Some(mut right) = right {
        // Avoid unnecessary parentheses warning
        while let Expr::Paren(paren) = *right {
            right = paren.expr;
        }
        args.push(*right);
    }
    ExprMethodCall {
        attrs,
        receiver: left,
        dot_token: Dot { spans: [span] },
        method: Ident::new(ident, span),
        turbofish: None,
        paren_token: Paren(span),
        args,
    }
}

fn empty_expr() -> Expr {
    Expr::Verbatim(Default::default())
}

pub fn empty_expr_binary() -> ExprBinary {
    ExprBinary {
        attrs: Vec::new(),
        left: Box::new(empty_expr()),
        op: BinOp::Add(Default::default()),
        right: Box::new(empty_expr()),
    }
}

pub fn empty_expr_unary() -> ExprUnary {
    ExprUnary {
        attrs: Vec::new(),
        op: UnOp::Neg(Default::default()),
        expr: Box::new(empty_expr()),
    }
}
