use crate::lexer::Lexer;
use crate::tok::TokKind;
use crate::ScriptMappings;
use chompy::diagnostics::{Builder, Result};
use chompy::lex::{Tok, Token};
use chompy::utils::Location;
use regex::Regex;

pub fn parse(source: &str, script_mappings: &ScriptMappings) -> Option<String> {
    let log_regex =
        Regex::new(r"gml_(?:Object|Script).+?(?::\d+|\(line \d+\))").unwrap();
    log_regex.find(source).map(|v| v.as_str()).and_then(|log| {
        parse_log(log, script_mappings)
            .ok()
            .map(|parsed_log| source.to_string().replace(log, &parsed_log))
    })
}

pub fn parse_log(source: &str, script_mappings: &ScriptMappings) -> Result<String> {
    fn take(toks: &mut Vec<Tok<TokKind>>) -> Result<TokKind> {
        if !toks.is_empty() {
            Ok(toks.remove(0).kind)
        } else {
            Err(ParseError("unexpected end".into()).into())
        }
    }

    // Lex the input
    let lexer = Lexer::new(Box::leak(Box::new(source.to_string())), 0); // hm.
    let mut toks: Vec<Tok<TokKind>> = lexer
        .collect::<std::result::Result<_, chompy::lex::LexError>>()
        .unwrap();
    toks.reverse();

    // The last token is always the line number
    let TokKind::LineNumber(line_number) = take(&mut toks)? else {
        return Err(ParseError("could not find line number".into()).into());
    };

    // If the next token is an event, then this is an object log. Otherwise, it is a script log.
    let next = take(&mut toks)?;

    // If the token after the line number is an object event, then this is an object
    if let TokKind::EventName(event) = next {
        // Find an object name
        let mut obj = String::new();
        for tok in toks.iter().skip(1) {
            if tok.kind() == TokKind::ObjectMarker {
                break;
            }
            obj = format!("{}{obj}", tok);
        }

        Ok(format!("objects/{obj}/{event}.gml:{line_number}:0"))
    } else {
        // Otherwise this is a script, and we'll work backwards to find out which it is
        let mut possibilities = vec![];
        let mut working_name = next.to_string();
        for tok in toks {
            if let Some(script) = script_mappings.get(&working_name) {
                possibilities.push(script);
            }
            working_name = format!("{}{working_name}", tok);
        }

        if let Some(script) = possibilities.last() {
            Ok(format!("scripts/{script}/{script}.gml:{line_number}:0"))
        } else {
            Err(ParseError("Could not find a matching script".into()).into())
        }
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
