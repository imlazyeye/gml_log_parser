# gml_log_parser

A small library for parsing the convoluted callstacks GameMaker outputs and translating them into VSC-friendly paths to the file/line in question.

```js
// Before
 "gml_Script_anon@9061@anon@9032_anon@9004_anon@8977_gml_Object_obj_example_Create_0:288"
 "gml_Object_obj_example_Step_2:7"
 "gml_Script_anon@11365@foobar_ScriptExample:347",

// After
"objects/obj_example/Create_0.gml:288:0"
"objects/obj_example/Step_2.gml:7:0"
"script/ScriptExample/ScriptExample.gml:347:0"
```
