//! # Aio IR
//!
//! Each [`Module`] represents a "package" of truths about concepts.

/// Represents a package of truths about concepts defined herein or elsewhere.
pub struct Module {
    // TODO: Use a stable-index, generational arena.
    pub declarations: Vec<Symbol>,
}

impl Module {
    /// Evaluates an expression in the context of this module.
    ///
    /// This is the entry point for all compilation as a program is compiled
    /// by evaluating `Compile::compile(main)`.
    ///
    /// A program might be written in this way:
    ///
    /// ```
    /// let main = print(42)
    /// ```
    ///
    /// Then executing `main` yields `PrintEffect(T)`
    ///
    /// `PrintEffect(x)` implements `Compile`, which is effectively serialization for effects.
    /// The effect is then serialized into an executable.
    pub fn query(&self, expression: &Expression) -> QueryResult {
        expression.evaluate(&self)
    }
}

pub enum QueryResult {
    Ok(Value),
    Err(Vec<QueryError>),
}

pub enum QueryError {
    /// The set of facts expressed in this module does not guarantee a singular, specific value for this expression.
    UnconstrainedUniverse,
    /// A symbol wasn't recognised and thus treated as an unconstrained entity.
    UnknownSymbol,
}

/// A symbol — a base language feature that allows referring to the same entity in multiple places.
///
/// Usually defined like `let x = ...` and used as `print(x)`;
pub struct Symbol {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // A simple value.
    Value(Value),
    // `let x`
    Capture(Identifier),
    // `a(b)`
    Apply(Box<Application>),
    // `a`
    SymbolRef(Identifier),
    // An error, a little viral node of an expression
    // that makes expressions "not count."
    //
    // Certain IRs will still be partially evaluatable even if there are errors,
    // so this expression kind marks expressions to be ignored!
    Error,
}

impl Expression {
    pub fn evaluate(&self, _module: &Module) -> QueryResult {
        match self {
            Expression::Error => QueryResult::Ok(Value::Error),
            Expression::Value(value) => QueryResult::Ok(value.clone()),
            Expression::Capture(_) => QueryResult::Ok(Value::Primitive(PrimitiveValue::Void)),

            // For certain builtins, there are hardcoded applications.
            // For example, integer sum, etc.
            Expression::Apply(_) => todo!(),

            // Referring to a symbol in an a block such as a {} block or a module.
            Expression::SymbolRef(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Application {
    pub callable: Expression,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Primitive(PrimitiveValue),
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveValue {
    Integer(PrimitiveInteger),
    Bool(bool),
    Print(Box<Expression>),
    Void,

    // -- Some primitive functions! --
    IntegerAdd,
    IntegerSubtract,
    IntegerMultiply,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveInteger {
    pub value: u64,
    pub sign: bool,
    pub bits: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Identifier(pub std::sync::Arc<str>);
