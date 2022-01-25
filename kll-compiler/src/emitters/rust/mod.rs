use std::fs;
use std::path::Path;

use crate::{KllState, Value};

pub fn datatype(s: &str) -> &str {
    if s.parse::<i32>().is_ok() {
        "i32"
    } else if s.parse::<f32>().is_ok() {
        "f32"
    } else {
        "&str" // default to string
    }
}

pub fn defines(kll_data: &KllState) -> String {
    let mut defines = String::new();
    for (name, value) in &kll_data.variables {
        if let Some(cname) = kll_data.defines.get(name) {
            defines += &match value {
                Value::List(vec) => format!(
                    "pub const {}: &[{}] = &[{}];\n",
                    cname,
                    datatype(vec[0]),
                    vec.join(", ")
                ),
                Value::Single(s) => format!("pub const {}: {} = {};\n", cname, datatype(s), s),
            };
        } else {
            // Not exposed externally
        }
    }

    defines
}

pub fn write(file: &Path, kll_data: &KllState) {
    let content = format!(" ///// DEFINES /////\n{}\n", defines(kll_data));
    fs::write(file, content).unwrap();
}
