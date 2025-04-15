#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![doc = include_str!("../README.md")]

use block::{Checked, MathBlock, Overflowing, Propagating, Saturating, Wrapping};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, visit_mut::VisitMut, Block, Stmt};
use visitor::MathBlockVisitor;

mod block;
mod expr;
mod visitor;

/// Defines a block of code where all mathematical operations are performed using checked methods.
///
/// If any operation overflows, it will panic with a corresponding error message.
///
/// # Example
///
/// ```rust
/// use overf::checked;
///
/// checked! {
///     let a = 10usize + 5usize;
///     let b = 20usize - 10usize;
///     let c = 3usize * 7usize;
/// }
/// ```
#[proc_macro]
pub fn checked(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand::<Checked>(input)
}

/// Defines a block of code where all mathematical operations use overflowing methods.
///
/// When an operation overflows, it will not panic; instead, it will return the result of the operation, wrapping around if necessary.
///
/// # Example
///
/// ```rust
/// use overf::overflowing;
///
/// overflowing! {
///     let a = 10usize + 5usize;
///     let b = 200usize - 300usize; // Overflows
/// }
/// ```
#[proc_macro]
pub fn overflowing(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand::<Overflowing>(input)
}

/// Defines a block of code where all mathematical operations use saturating methods.
///
/// When an operation would overflow, it will instead return the maximum (or minimum) value of the type.
///
/// # Example
///
/// ```rust
/// use overf::saturating;
///
/// saturating! {
///     let a = usize::MAX + 1; // Saturates to usize::MAX
///     let b = usize::MIN - 1; // Saturates to usize::MIN
/// }
/// ```
#[proc_macro]
pub fn saturating(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand::<Saturating>(input)
}

/// Defines a block of code where all mathematical operations use checked methods.
/// If any operation results in an overflow, it will return `None`, propagating the error using the `?` operator.
///
/// This macro is useful when you want to handle potential overflows in a concise way, allowing the calling function
/// to manage the error or terminate early.
///
/// # Example
///
/// ```rust
/// use overf::propagating;
///
/// fn example() -> Option<usize> {
///     propagating! {
///         // Returns `None` if any operation fails.
///         let a = 10usize + 5usize;
///         let b = 20usize - 10usize;
///         let c = 3usize * 7usize;
///         let d = 21usize / 3usize;
///         let e = a + b;
///         Some(e)
///     }
/// }
///
/// example().unwrap();
/// ```
#[proc_macro]
pub fn propagating(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand::<Propagating>(input)
}

/// Defines a block of code where all mathematical operations use wrapping methods.
///
/// When an operation overflows, it will not panic; instead, it will return the result of the operation, wrapping around if necessary.
///
/// # Example
///
/// ```rust
/// use overf::wrapping;
///
/// wrapping! {
///     let a = 10usize + 5usize;
///     let b = 200usize - 300usize; // Overflows
/// }
/// ```
#[proc_macro]
pub fn wrapping(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand::<Wrapping>(input)
}

/// Resets the overflow behavior to the default behavior of Rust.
///
/// This is useful when you want to exit a block with custom overflow handling and revert to the standard behavior.
///
/// # Example
///
/// ```rust
/// use overf::{checked, default};
///
/// checked! {
///     let a = 10usize + 5usize; // checked
///         
///     default! {
///         let b = a + 1000; // Uses default behavior (may panic or overflow)
///     }
/// }
/// ```
#[proc_macro]
pub fn default(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}

fn expand<B: MathBlock>(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    struct Stmts(Vec<Stmt>);

    impl Parse for Stmts {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Block::parse_within(input).map(Self)
        }
    }

    let stmts = parse_macro_input!(input as Stmts);
    match try_expand::<B>(stmts.0) {
        Ok(res) => res.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn try_expand<B: MathBlock>(stmts: Vec<Stmt>) -> syn::Result<proc_macro2::TokenStream> {
    let mut res = proc_macro2::TokenStream::new();
    for mut stmt in stmts {
        MathBlockVisitor::<B>::new().visit_stmt_mut(&mut stmt);
        res.extend(quote! { #stmt });
    }
    Ok(res)
}
