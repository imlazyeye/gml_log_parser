use chompy::lex::{Token, TokenKind};
use chompy::{
    diagnostics::Result,
    lex::{CharStream, Lex, LexError, Tok},
    utils::*,
};
use std::collections::HashMap;
use std::fmt::Display;

use crate::TokKind;

pub struct Lexer {
    source: &'static str,
    char_stream: CharStream,
    file_id: usize,
}

impl Lexer {
    /// Creates a new Lexer, taking a string of fog source.
    pub fn new(source: &'static str, file_id: FileId) -> Self {
        Self {
            source,
            char_stream: CharStream::new(source),
            file_id,
        }
    }
}

impl Iterator for Lexer {
    type Item = std::result::Result<Tok<TokKind>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lex() {
            Ok(Some(item)) => Some(Ok(item)),
            Err(e) => Some(Err(e)),
            _ => None,
        }
    }
}

impl Lex<Tok<TokKind>, TokKind> for Lexer {
    fn source(&self) -> &'static str {
        self.source
    }

    fn file_id(&self) -> FileId {
        self.file_id
    }

    fn char_stream(&mut self) -> &mut CharStream {
        &mut self.char_stream
    }

    fn construct_ident(&mut self) -> Option<&'static str> {
        let test = |c: char| -> bool { c.is_alphanumeric() };
        self.char_stream()
            .match_peek_with(test)
            .then(|| self.construct(test))
    }

    fn lex(&mut self) -> std::result::Result<Option<Tok<TokKind>>, LexError> {
        let start_pos = self.char_stream.position();

        let kind = if let Some(num) = self.construct_integer(false) {
            TokKind::Number(num)
        } else if let Some(ident) = self.construct_ident() {
            match ident {
                "gml" => TokKind::Gml,
                "Script" => TokKind::Script,
                "Object" => TokKind::Object,
                lexeme => TokKind::Ident(lexeme),
            }
        } else {
            let Some(chr) = self.chomp() else {
                return Ok(None);
            };
            match chr {
                '@' => TokKind::At,
                '_' => TokKind::Underscore,
                ':' => TokKind::Colon,
                invalid => {
                    // this is chill, I promise
                    let tmp = Box::leak(Box::new([0u8; 4]));
                    let invalid = invalid.encode_utf8(tmp);
                    TokKind::Invalid(invalid)
                }
            }
        };

        Ok(Some(Tok::new(
            kind,
            Location::new(
                self.file_id,
                Span::new(start_pos, self.char_stream.position()),
            ),
        )))
    }
}
