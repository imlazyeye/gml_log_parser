use chompy::diagnostics::{Builder, Result};
use chompy::lex::Tok;
use chompy::utils::Location;
use std::collections::HashMap;

use crate::TokKind;

pub fn parse(
    mut toks: Vec<Tok<TokKind>>,
    script_mappings: HashMap<String, String>,
) -> Result<String> {
    fn take(toks: &mut Vec<Tok<TokKind>>) -> Result<TokKind> {
        if !toks.is_empty() {
            Ok(toks.remove(0).kind)
        } else {
            Err(ParseError("unexpected end".into()).into())
        }
    }

    // The first token tells us what this is
    let marker = take(&mut toks)?;

    // And the last token should always be a line number
    toks.reverse();
    let TokKind::LineNumber(line_number) = take(&mut toks)? else {
        return Err(ParseError("could not find line number".into()).into());
    };

    // And the first is a marker that will tell us what this is
    match marker {
        TokKind::ScriptMarker => {
            // We will continue to try to build possible script names over and over. The longest one we
            // find is the winner.
            let mut possibilities = vec![];
            let mut working_name = String::new();
            for tok in toks {
                working_name = format!("{}{working_name}", tok);
                if let Some(script) = script_mappings.get(&working_name) {
                    possibilities.push(script);
                }
            }

            if let Some(script) = possibilities.last() {
                Ok(format!("scripts/{script}/{script}.gml:{line_number}:0"))
            } else {
                Err(ParseError("Could not find a matching script".into()).into())
            }
        }
        TokKind::ObjectMarker => {
            let TokKind::EventName(event) = take(&mut toks).unwrap() else {
                return Err(ParseError("Could not find an object event".into()).into());
            };

            // Find an object name
            let mut obj = String::new();
            for tok in toks.iter().skip(1) {
                obj = format!("{}{obj}", tok);
            }

            Ok(format!("objects/{obj}/{event}.gml:{line_number}:0"))
        }
        _ => Err(ParseError("Could not find a marker".into()).into()),
    }
}

#[derive(Debug)]
struct ParseError(String);
chompy::define_error!(
    ParseError {
        fn build(&self, builder: Builder) -> Builder {
            builder
                .title(self.0.as_str())
        }
        fn location(&self) -> Location {
            Location::default() // we don't bother with locations for this
        }
    }
);
