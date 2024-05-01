use chompy::lex::TokenKind;

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokKind {
    ScriptMarker,
    ObjectMarker,
    Underscore,
    LineNumber(u64),
    EventName(&'static str),
    Ident(&'static str),
    Invalid(char),
}

impl Display for TokKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokKind::ScriptMarker => "gml_Script_",
            TokKind::ObjectMarker => "gml_Object_",
            TokKind::Underscore => "_",
            TokKind::LineNumber(i) => return f.pad(&format!(":{i}")),
            TokKind::Ident(i) => i,
            TokKind::EventName(name) => return f.pad(name),
            TokKind::Invalid(name) => return f.pad(&format!("{name}")),
        };
        f.pad(s)
    }
}

impl TokenKind for TokKind {}
