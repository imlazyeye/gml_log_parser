macro_rules! check {
    ($name:ident, $($source:expr => $expected:expr),* $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let script_mappings = crate::ScriptMappings::new(
                vec![
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
                .collect()
            );
            $(
                pretty_assertions::assert_eq!(
                    crate::parse::parse($source.into(), &script_mappings).unwrap(),
                    $expected.to_string()
                );
            )*
        }
    };
}

check!(
    create_event,
    "gml_Object_ObjObject_Create_0:7" => "objects/ObjObject/Create_0.gml:7:0",
    "gml_Object_obj_object_Create_0:7" => "objects/obj_object/Create_0.gml:7:0",
    "gml_Object___obj__object_Create_0:7" => "objects/__obj__object/Create_0.gml:7:0",
);
check!(
    global_script_by_name,
    "gml_Script_ScriptFunction:102" => "scripts/ScriptFile/ScriptFile.gml:102:0",
    "gml_Script_script_function:102" => "scripts/script_file/script_file.gml:102:0",
    "gml_Script__script__function_:102" => "scripts/___scriptfile/___scriptfile.gml:102:0",

);
check!(
    constructor_method,
    "gml_Script_method@ScriptFunction_ScriptFile:296" => "scripts/ScriptFile/ScriptFile.gml:296:0",
    "gml_Script_method@script_function_script_file:296" => "scripts/script_file/script_file.gml:296:0",
    "gml_Script_method@_script__function____scriptfile:296" => "scripts/___scriptfile/___scriptfile.gml:296:0",
);
check!(
    object_method,
    "gml_Script_ObjectFunction@gml_Object_ObjObject_Create_0:273" => "objects/ObjObject/ObjObject.gml:273:0",
    "gml_Script_object_function@gml_Object_obj_object_Create_0:273" => "objects/obj_object/obj_object.gml:273:0",
    "gml_Script_object_function@gml_Object___obj__object_Create_0:273" => "objects/__obj__object/__obj__object.gml:273:0"
);
check!(
    utter_nonsense,
    "gml_Script_foo_bar_bee@anon@3215@anon@5923@__struct__542_ScriptFunction_ScriptFile:296" => "scripts/ScriptFile/ScriptFile.gml:296:0",
    "gml_Script_foo_bar_bee@anon@3215@anon@5923@__struct__542_script_function_script_file:296" => "scripts/script_file/script_file.gml:296:0",
    "gml_Script_foo_bar_bee@anon@3215@anon@5923@__struct__542__script__function____scriptfile:296" => "scripts/___scriptfile/___scriptfile.gml:296:0",
);
check!(
    anon_fn_in_constructor_method,
    "gml_Script_anon@83894@ScriptFunction_ScriptFile:2649" => "scripts/ScriptFile/ScriptFile.gml:2649:0",
    "gml_Script_anon@83894@script_function_script_file:2649" => "scripts/script_file/script_file.gml:2649:0",
    "gml_Script_anon@83894@_script__function___scriptfile:2649" => "scripts/___scriptfile/___scriptfile.gml:2649:0",
);
check!(
    crash_log,
    "gml_Script_ScriptFunction (line 254)" => "scripts/ScriptFile/ScriptFile.gml:254:0",
    "gml_Script_script_function (line 254)" => "scripts/script_file/script_file.gml:254:0",
    "gml_Script__script__function_ (line 254)" => "scripts/___scriptfile/___scriptfile.gml:254:0",
);
