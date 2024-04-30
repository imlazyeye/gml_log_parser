

use chompy::lex::{Token, TokenKind};
use chompy::{
    diagnostics::Result,
    lex::{CharStream, Lex, LexError, Tok},
    utils::*,
};
use std::collections::HashMap;
use std::fmt::Display;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokKind {
    Gml,
    Script,
    Object,
    Anon,
    At,
    Colon,
    Underscore,
    Number(i64),
    Ident(&'static str),
    Invalid(&'static str),
}

impl Display for TokKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokKind::Gml => "gml",
            TokKind::Script => "Script",
            TokKind::Object => "Object",
            TokKind::Anon => "anon",
            TokKind::At => "@",
            TokKind::Colon => ":",
            TokKind::Underscore => "_",
            TokKind::Number(n) => return f.pad(&n.to_string()),
            TokKind::Ident(i) => i,
            TokKind::Invalid(_) => todo!(),
        };
        f.pad(s)
    }
}

impl TokenKind for TokKind {}