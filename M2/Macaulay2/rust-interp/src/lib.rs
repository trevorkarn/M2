pub mod ast;
pub mod convert;
pub mod ffi;
pub mod lexer;
pub mod parser;

// New modules replacing D files
pub mod arithmetic;
pub mod atomic;
pub mod interrupts;
pub mod m2_types;
pub mod generate_c_wrapper;
pub mod c_ffi_exports;

pub use ast::{ParseTree, Position, Token, TokenKind, Word};
pub use convert::{Code, convert};
pub use ffi::{m2_rust_free_string, m2_rust_parse_and_convert};
pub use lexer::{tokenize, LexError};
pub use parser::{parse, ParseError};

pub type ParseResult<T> = Result<T, ParseError>;
