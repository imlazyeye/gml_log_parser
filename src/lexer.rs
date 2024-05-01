use chompy::{
    lex::{CharStream, Lex, LexError, Tok},
    utils::*,
};

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

        let kind = if let Some(ident) = self.construct_ident() {
            match ident {
                "gml" if self.chomp_pattern("_Object_") => TokKind::ObjectMarker,
                "gml" if self.chomp_pattern("_Script_") => TokKind::ScriptMarker,
                lexeme
                    if GML_EVENTS.contains(&lexeme)
                        && self.char_stream.match_peek('_')
                        && self.char_stream.peek_while(|v| v.is_numeric()) =>
                {
                    self.char_stream.chomp_peeks();
                    TokKind::EventName(
                        self.char_stream
                            .slice(start_pos..self.char_stream.position()),
                    )
                }
                lexeme => TokKind::Ident(lexeme),
            }
        } else {
            let Some(chr) = self.chomp() else {
                return Ok(None);
            };
            match chr {
                '_' => TokKind::Underscore,
                ':' => {
                    let number = self.construct_integer(false).unwrap();
                    TokKind::LineNumber(number as u64)
                }
                '(' if self.chomp_pattern("line ") => {
                    let number = self.construct_integer(false).unwrap();
                    self.chomp(); // )
                    TokKind::LineNumber(number as u64)
                }
                chr if chr.is_whitespace() => return self.lex(),
                invalid => TokKind::Invalid(invalid),
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

const GML_EVENTS: &[&str] = &[
    "Alarm",
    "CleanUp",
    "Create",
    "Destroy",
    "Draw",
    "Gesture",
    "Keyboard",
    "KeyRelease",
    "Other",
    "Step",
];
