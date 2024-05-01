// #![warn(missing_docs)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::print_stdout)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::similar_names)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::panic)]

mod lexer;
mod parse;
mod tok;
pub use lexer::*;
pub use parse::*;
pub use tok::*;

#[cfg(test)]
mod tests;
