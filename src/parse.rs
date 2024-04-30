use chompy::lex::{Token, TokenKind};
use chompy::{
    diagnostics::Result,
    lex::{CharStream, Lex, LexError, Tok},
    utils::*,
};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;

use crate::TokKind;

pub fn parse(
    mut toks: Vec<Tok<TokKind>>,
    script_mappings: HashMap<String, String>,
) -> Option<String> {
    fn match_take(kind: TokKind, toks: &mut Vec<Tok<TokKind>>) -> Option<Tok<TokKind>> {
        if let Some(tok) = toks.first() {
            if tok.kind() == kind {
                return Some(toks.remove(0));
            }
        }

        None
    }

    fn take(toks: &mut Vec<Tok<TokKind>>) -> Result<Tok<TokKind>> {
        if let Some(tok) = toks.first() {
            return Ok(toks.remove(0));
        }

        Err(todo!())
    }

    fn require(kind: TokKind, toks: &mut Vec<Tok<TokKind>>) -> Result<()> {
        if let Some(tok) = toks.first() {
            if tok.kind() == kind {
                toks.remove(0);
                return Ok(());
            }
        }

        return panic!("Failed on {}", kind);
    }

    const OBJECT_FILES: &[&str] = &["Create_0", "Step_0", "Step_1", "Step_2"];

    println!("input: {:?}", toks.iter().map(|v| v.to_string()).join(""));

    // Reverse the stream
    toks.reverse();
    println!("reved: {:?}", toks.iter().map(|v| v.to_string()).join(""));

    // The last token should always be a number...
    let Ok(TokKind::Number(line_number)) = take(&mut toks).map(|v| v.kind()) else {
        panic!()
    };

    // Followed by a colon
    toks.remove(0);
    println!("no line: {:?}", toks.iter().map(|v| v.to_string()).join(""));

    let mut last_was_underscore = false;
    toks = toks
        .into_iter()
        .rev()
        .take_while(|token| {
            if token.kind() == TokKind::Gml && last_was_underscore {
                return false;
            }
            last_was_underscore = token.kind() == TokKind::Underscore;
            true
        })
        .collect();

    // Spin it back round
    println!(
        "filtered: {:?}",
        toks.iter().map(|v| v.to_string()).join("")
    );

    // Now we figure out what kind of source we're working with
    require(TokKind::Gml, &mut toks).unwrap();
    require(TokKind::Underscore, &mut toks).unwrap();

    if match_take(TokKind::Script, &mut toks).is_some() {
        require(TokKind::Underscore, &mut toks).unwrap();

        // We will continue to try to build possible script names over and over. The longest one we
        // find is the winner.
        let mut possibilities = vec![];
        let mut working_name = String::new();
        toks.reverse();
        for tok in toks {
            working_name = format!("{}{working_name}", tok);
            println!("working_name: {working_name}");
            if let Some(script) = script_mappings.get(&working_name) {
                possibilities.push(script);
            }
        }

        if let Some(script) = possibilities.last() {
            return Some(format!("scripts/{script}/{script}.gml:{line_number}"));
        } else {
            panic!()
        }
    } else if match_take(TokKind::Object, &mut toks).is_some() {
        require(TokKind::Underscore, &mut toks).unwrap();
        toks.reverse();

        // Find an event name
        let mut event = String::new();
        while let Some(tok) = toks.first() {
            let tok = toks.remove(0);
            event = format!("{}{event}", tok);
            // println!("working_name: {working_name}");
            if OBJECT_FILES.contains(&event.as_ref()) {
                break;
            }
        }

        println!("event: {event}");

        println!("{}", toks.iter().join(""));

        // Find an object name
        require(TokKind::Underscore, &mut toks).unwrap();
        let mut obj = String::new();
        for tok in toks {
            if tok.kind() == TokKind::Object {
                break;
            }

            obj = format!("{}{obj}", tok);
            println!("obj: {obj}");
        }

        println!("obj: {obj}");

        return Some(format!("objects/{obj}/{event}.gml:{line_number}"));
    } else {
        panic!(); // what?
    }
}
