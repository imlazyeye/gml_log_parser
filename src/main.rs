use chompy::lex::{Token, TokenKind};
use chompy::{
    diagnostics::Result,
    lex::{CharStream, Lex, LexError, Tok},
    utils::*,
};
use std::fmt::Display;
use std::iter::Peekable;

fn main() {
    println!("Hello, world!");
}

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

fn parse(mut toks: Vec<Tok<TokKind>>) -> Result<String> {
    fn matching_script(name: &str) -> Option<String> {
        todo!()
    }

    fn match_take(kind: TokKind, toks: &mut Vec<Tok<TokKind>>) -> Option<Tok<TokKind>> {
        if let Some(tok) = toks.first() {
            if tok.kind() == kind {
                return Some(toks.remove(0));
            }
        }

        None
    }

    fn require(kind: TokKind, toks: &mut Vec<Tok<TokKind>>) -> Result<()> {
        if let Some(tok) = toks.first() {
            if tok.kind() == kind {
                toks.remove(0);
                return Ok(());
            }
        }

        return todo!();
    }

    // The last token should always be a number...
    let Some(TokKind::Number(line_number)) = toks.pop().map(|v| v.kind()) else {
        panic!()
    };

    // Followed by a colon
    require(TokKind::Colon, &mut toks);

    // Now we will traverse our tokens in reverse until we encounter `gml`
    let mut rev_stream = toks.into_iter().rev();
    toks = rev_stream
        .take_while(|v| v.kind() != TokKind::Gml)
        .collect();

    // Reverse us back
    toks.reverse();

    // The underscore from gml_
    require(TokKind::Underscore, &mut toks);

    // Now we figure out what kind of source we're working with
    let path = if match_take(TokKind::Script, &mut toks).is_some() {
        require(TokKind::Underscore, &mut toks);

        // We will continue to try to build possible script names over and over. The longest one we
        // find is the winner.
        let mut possibilities = vec![];
        let mut working_name = String::new();
        while let Some(tok) = toks.iter().next() {
            working_name = format!("{}{working_name}", toks.remove(0));
            if let Some(script) = matching_script(&working_name) {
                possibilities.push(script);
            }
        }

        if let Some(script) = possibilities.last() {
            Some(format!("scripts/{script}/{script}.gml:{line_number}"));
        } else {
            panic!()
        }
        
    } else if match_take(TokKind::Object, &mut toks).is_some() {
        // EXAMPLE: Object_Setup_Create_0
    } else {
        panic!(); // what?
    };

    Ok(todo!())
}
