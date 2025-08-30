use syn::{
    BinOp, Expr, ExprField, ExprLit, ExprTry, Index, Lit, LitStr, Member, UnOp,
    spanned::Spanned,
    token::{Dot, Question},
};

use crate::expr::method_call;

pub trait MathBlock {
    fn method_ident_bin(op: BinOp) -> Option<&'static str>;
    fn method_ident_un(op: UnOp) -> Option<&'static str>;
    fn finalize_bin(expr: Expr, op: BinOp) -> Expr {
        let _ = op;
        expr
    }
    fn finalize_un(expr: Expr, op: UnOp) -> Expr {
        let _ = op;
        expr
    }
}

pub struct Checked;
pub struct Overflowing;
pub struct Saturating;
pub struct Propagating;
pub struct Wrapping;

impl MathBlock for Checked {
    fn method_ident_bin(op: BinOp) -> Option<&'static str> {
        match op {
            BinOp::Add(_) | BinOp::AddAssign(_) => Some("checked_add"),
            BinOp::Sub(_) | BinOp::SubAssign(_) => Some("checked_sub"),
            BinOp::Mul(_) | BinOp::MulAssign(_) => Some("checked_mul"),
            BinOp::Div(_) | BinOp::DivAssign(_) => Some("checked_div"),
            BinOp::Rem(_) | BinOp::RemAssign(_) => Some("checked_rem"),
            BinOp::Shl(_) | BinOp::ShlAssign(_) => Some("checked_shl"),
            BinOp::Shr(_) | BinOp::ShrAssign(_) => Some("checked_shr"),
            _ => None,
        }
    }

    fn method_ident_un(op: UnOp) -> Option<&'static str> {
        match op {
            UnOp::Neg(_) => Some("checked_neg"),
            _ => None,
        }
    }

    fn finalize_bin(expr: Expr, op: BinOp) -> Expr {
        let error = match op {
            BinOp::Add(_) | BinOp::AddAssign(_) => Some("attempt to add with overflow"),
            BinOp::Sub(_) | BinOp::SubAssign(_) => Some("attempt to subtract with overflow"),
            BinOp::Mul(_) | BinOp::MulAssign(_) => Some("attempt to multiply with overflow"),
            BinOp::Div(_) | BinOp::DivAssign(_) => Some("attempt to divide by zero"),
            BinOp::Rem(_) | BinOp::RemAssign(_) => {
                Some("attempt to calculate the remainder with a divisor of zero")
            }
            BinOp::Shl(_) | BinOp::ShlAssign(_) => Some("attempt to shift left with overflow"),
            BinOp::Shr(_) | BinOp::ShrAssign(_) => Some("attempt to shift right with overflow"),
            _ => None,
        };

        let call = if let Some(error) = error {
            method_call(
                "expect",
                Box::new(expr),
                Some(Box::new(Expr::Lit(ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Str(LitStr::new(error, op.span())),
                }))),
                op.span(),
                Vec::new(),
            )
        } else {
            method_call("unwrap", Box::new(expr), None, op.span(), Vec::new())
        };
        Expr::MethodCall(call)
    }

    fn finalize_un(expr: Expr, op: UnOp) -> Expr {
        let error = match op {
            UnOp::Neg(_) => Some("attempt to negate with overflow"),
            _ => None,
        };

        let call = if let Some(error) = error {
            method_call(
                "expect",
                Box::new(expr),
                Some(Box::new(Expr::Lit(ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Str(LitStr::new(error, op.span())),
                }))),
                op.span(),
                Vec::new(),
            )
        } else {
            method_call("unwrap", Box::new(expr), None, op.span(), Vec::new())
        };
        Expr::MethodCall(call)
    }
}

impl MathBlock for Overflowing {
    fn method_ident_bin(op: BinOp) -> Option<&'static str> {
        match op {
            BinOp::Add(_) | BinOp::AddAssign(_) => Some("overflowing_add"),
            BinOp::Sub(_) | BinOp::SubAssign(_) => Some("overflowing_sub"),
            BinOp::Mul(_) | BinOp::MulAssign(_) => Some("overflowing_mul"),
            BinOp::Div(_) | BinOp::DivAssign(_) => Some("overflowing_div"),
            BinOp::Rem(_) | BinOp::RemAssign(_) => Some("overflowing_rem"),
            BinOp::Shl(_) | BinOp::ShlAssign(_) => Some("overflowing_shl"),
            BinOp::Shr(_) | BinOp::ShrAssign(_) => Some("overflowing_shr"),
            _ => None,
        }
    }

    fn method_ident_un(op: UnOp) -> Option<&'static str> {
        match op {
            UnOp::Neg(_) => Some("overflowing_neg"),
            _ => None,
        }
    }

    fn finalize_bin(expr: Expr, op: BinOp) -> Expr {
        Expr::Field(ExprField {
            attrs: Vec::new(),
            base: Box::new(expr),
            dot_token: Dot(op.span()),
            member: Member::Unnamed(Index::from(0)),
        })
    }

    fn finalize_un(expr: Expr, op: UnOp) -> Expr {
        Expr::Field(ExprField {
            attrs: Vec::new(),
            base: Box::new(expr),
            dot_token: Dot(op.span()),
            member: Member::Unnamed(Index::from(0)),
        })
    }
}

impl MathBlock for Saturating {
    fn method_ident_bin(op: BinOp) -> Option<&'static str> {
        match op {
            BinOp::Add(_) | BinOp::AddAssign(_) => Some("saturating_add"),
            BinOp::Sub(_) | BinOp::SubAssign(_) => Some("saturating_sub"),
            BinOp::Mul(_) | BinOp::MulAssign(_) => Some("saturating_mul"),
            BinOp::Div(_) | BinOp::DivAssign(_) => Some("saturating_div"),
            _ => None,
        }
    }

    fn method_ident_un(op: UnOp) -> Option<&'static str> {
        match op {
            UnOp::Neg(_) => Some("saturating_neg"),
            _ => None,
        }
    }
}

impl MathBlock for Propagating {
    fn method_ident_bin(op: BinOp) -> Option<&'static str> {
        Checked::method_ident_bin(op)
    }

    fn method_ident_un(op: UnOp) -> Option<&'static str> {
        Checked::method_ident_un(op)
    }

    fn finalize_bin(expr: Expr, op: BinOp) -> Expr {
        Expr::Try(ExprTry {
            attrs: Vec::new(),
            expr: Box::new(expr),
            question_token: Question(op.span()),
        })
    }

    fn finalize_un(expr: Expr, op: UnOp) -> Expr {
        Expr::Try(ExprTry {
            attrs: Vec::new(),
            expr: Box::new(expr),
            question_token: Question(op.span()),
        })
    }
}

impl MathBlock for Wrapping {
    fn method_ident_bin(op: BinOp) -> Option<&'static str> {
        match op {
            BinOp::Add(_) | BinOp::AddAssign(_) => Some("wrapping_add"),
            BinOp::Sub(_) | BinOp::SubAssign(_) => Some("wrapping_sub"),
            BinOp::Mul(_) | BinOp::MulAssign(_) => Some("wrapping_mul"),
            BinOp::Div(_) | BinOp::DivAssign(_) => Some("wrapping_div"),
            BinOp::Rem(_) | BinOp::RemAssign(_) => Some("wrapping_rem"),
            BinOp::Shl(_) | BinOp::ShlAssign(_) => Some("wrapping_shl"),
            BinOp::Shr(_) | BinOp::ShrAssign(_) => Some("wrapping_shr"),
            _ => None,
        }
    }

    fn method_ident_un(op: UnOp) -> Option<&'static str> {
        match op {
            UnOp::Neg(_) => Some("wrapping_neg"),
            _ => None,
        }
    }
}
