use chompy::lex::{Token, TokenKind};
use chompy::{
    diagnostics::Result,
    lex::{CharStream, Lex, LexError, Tok},
    utils::*,
};
use std::collections::HashMap;
use std::fmt::Display;

macro_rules! check {
    ($name:ident, $source:expr => $expected:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let script_mappings: std::collections::HashMap<String, String> = vec![
                // Pascals
                ("ScriptFile", "ScriptFile"),
                ("ScriptFunction", "ScriptFile"),
                ("ObjObject", "ObjObject"),

                 // Snakes
                ("script_file", "script_file"),
                ("script_function", "script_file"),
                ("obj_object", "obj_object"),

                 // Odd underscores
                ("___scriptfile", "___scriptfile"),
                ("_script__function_", "___scriptfile"),
                ("__obj__object", "__obj__object"),
            ]
            .into_iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
            let lexer = crate::lexer::Lexer::new($source, 0);
            let toks: Vec<chompy::lex::Tok<crate::tok::TokKind>> = lexer
                .collect::<std::result::Result<_, chompy::lex::LexError>>()
                .unwrap();
            assert_eq!(
                crate::parse::parse(toks, script_mappings),
                Some($expected.to_string())
            );
        }
    };
}

check!(
    create_event,
    "gml_Object_ObjObject_Create_0:7" => "objects/ObjObject/Create_0.gml:7"
);
check!(
    global_script_by_name,
    "gml_Script_ScriptFunction:102" => "scripts/ScriptFile/ScriptFile.gml::102"
);
check!(
    constructor_method,
    "gml_Script_method@ScriptFunction_ScriptFile:296" => "scripts/ScriptFile/ScriptFile.gml:296"
);
check!(
    anon_fn_in_constructor_method,
    "gml_Script_anon@83894@ScriptFunction_ScriptFile:2649" => "scripts/ScriptFile/ScriptFile.gml:2649"
);
