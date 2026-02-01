//! # Aio
//!
//! A language where programs are defined in terms of invariants at compile-time,
//! and execution is a materalisation of a consistent world.
//!
//! Consider the classic JavaScript program:
//! ```ts
//! let x;
//!
//! x = 0;
//! console.log(x);
//! x = 2;
//! console.log(x);
//! ```
//!
//! If you're learning programming as a math nerd, the lines `x = 0` and `x = 2` might be strange at first.
//! They, together, seem to disregard the transitive property of equality (since 0 = 2 is false).
//!
//! Of course, in a procedural programming what that represents is _assignment_—muutation of a variable's content.
//! This is something that every programmer learns to work with, but that doesn't mean it's not eventually confusing.
//!
//! Mutability and memory aliasing lends to confusing programs where you can't tell if certain assumptions about
//! your programs (which we call invariants) hold. In Aio, the very building block of your program is invariants:
//! stuff that will remain true for the entirety of your program. And thus there's NO mutability anywhere.
//!
//! Whereas in a language like C, where each statement is an instruction which could be followed blindly,
//! an Aio file is not itself a program, it only _describes_ a program which the compiler generates.
//! _That program_ will do, yes, all sorts of state mutation and IO, of course, while benefitting from the
//! correctness enforced by Aio's compiler.
//!
//! ```aio
//! let x = 3
//!
//! fn main -> print(x)   # prints 3
//! ```

pub mod compile;
pub mod ir;
pub mod parse;
pub mod syntax;
