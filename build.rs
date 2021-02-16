use std::env;
use std::fs;
use std::path::Path;

use kll_rs;

fn main() {
    let file = fs::read_to_string("test.kll").expect("cannot read file");
    let kll_statements = kll_rs::parse(&file).unwrap();
    let kll_data = kll_statements.into_struct();

    let mut defines = String::new();
    for (name, value) in kll_data.defines {
        // TODO: Currently only text numerics are supported. Add arrays / enums / structs
        if let Ok(ival) = value.parse::<i32>() {
            defines += &format!("pub const {}: {} = {};\n", name, "i32", ival);
        } else {
            defines += &format!("pub const {}: {} = \"{}\";\n", name, "&str", value);
        }
    }

    let outfile = Path::new(&env::var("OUT_DIR").unwrap()).join("kll_defines.rs");
    fs::write(outfile, defines).unwrap();
}
