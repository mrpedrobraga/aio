//! # The Aio Syntax
//!
//! You can declare concepts using `let`, its name (a Symbol).
//!
//! ```aio
//! let x
//! ```
//!
//! To generate a program, the compiler will materialise a concept,
//! usually `main`.
//!
//! ```aio
//! let main = print("Hello, World")
//! ```
//!
//! The main concept needs not be a function, it can be anything that `impl Effect`.
//!
//! In Aio, concrete values are rarely used as function arguments. Instead, it's preferred to use
//! `abstracts` (those are called traits or interfaces in other languages).
//! For example, in the print effect as described before, it accepts any `impl Display`.
//!
//! You can implement abstracts for concepts that have no concrete value.
//!
//! ```aio
//! let x
//!
//! let x impl Display by (
//!     let display(&self) => "Hello, World"
//! )
//!
//! let main = print(x)   # fair game!
//! ```

use std::fmt::Debug;
pub mod implementations;

/// A reference to the range in the source file a branch of the syntax tree was parsed from.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryOp(Box<BinaryOperation>),
    Apply(Box<Application>),
    LExpression(LExpression),
}

#[derive(Debug, Clone)]
pub enum LExpression {
    Literal(Literal),
    SymbolRef(Identifier),
    Error,
}

#[derive(Debug, Clone)]
pub struct BinaryOperation {
    pub left: Spanned<Expression>,
    pub right: Spanned<Expression>,
    pub op: Spanned<BuiltinOperator>,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub callable: Spanned<Expression>,
    pub arguments: Vec<Spanned<Expression>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinOperator {
    Plus,  // +
    Minus, // -
    Times, // *
    Over,  // /
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(IntegerLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral {
    pub value: u64,
    pub sign: bool,
    pub bits: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Identifier(pub std::sync::Arc<str>);
