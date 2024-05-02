use regex::Regex;
use std::{collections::HashMap, fs::read_to_string, path::Path};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptMappings(HashMap<String, String>);
impl ScriptMappings {
    pub fn new(map: HashMap<String, String>) -> Self {
        Self(map)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let mut script_mappings = HashMap::new();

        let gml_files = WalkDir::new(path)
            .into_iter()
            .filter_map(|v| v.ok().map(|v| v.path().to_path_buf()))
            .filter_map(|v| {
                if v.extension().and_then(|v| v.to_str()) == Some("gml") {
                    let (Some(stem), Ok(file_data)) = (
                        v.file_stem().and_then(|v| v.to_str().map(String::from)),
                        read_to_string(v),
                    ) else {
                        return None;
                    };
                    Some((stem, file_data))
                } else {
                    None
                }
            });

        let func_finder = Regex::new(r#"(?m)^function\s+(\w+)"#).unwrap();

        for (stem, data) in gml_files {
            script_mappings.insert(stem.to_string(), stem.to_string());
            func_finder.captures_iter(&data).for_each(|v| {
                let (_, [func_name]) = v.extract();
                script_mappings.insert(func_name.to_string(), stem.to_string());
            })
        }

        Self(script_mappings)
    }
}

impl std::ops::Deref for ScriptMappings {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
