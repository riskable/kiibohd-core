use std::env;
use std::fs;
use std::path::Path;

use kll_rs;

fn main() {
    let file = fs::read_to_string("test.kll").expect("cannot read file");
    let kll_statements = kll_rs::parse(&file).unwrap();
    let kll_data = kll_statements.into_struct();

    let outfile = Path::new(&env::var("OUT_DIR").unwrap()).join("kll_defines.rs");
    kll_rs::emitters::rust::write(&outfile, &kll_data);
}
