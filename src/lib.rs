// #![warn(missing_docs)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::print_stdout)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::similar_names)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::undocumented_unsafe_blocks)]

mod lexer;
mod tok;
mod parse;
use chompy::lex::Tok;
pub use lexer::*;
pub use tok::*;
pub use parse::*;

#[cfg(test)]
mod tests;

