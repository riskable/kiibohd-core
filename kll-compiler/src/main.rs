use kll::KllDatastore;
use std::env;
use std::fs;

fn main() {
    let file = fs::read_to_string("test.kll").expect("cannot read file");
    let kll = kll::parse(&file).unwrap();
    println!("{}", kll);
    let kll_state = kll.into_struct();
    println!("{:?}", kll_state);
    let kll_data = KllDatastore::new(&kll_state);
    println!("{:?}", kll_data);

    let outfile = env::current_dir().unwrap().join("generatedKeymap.h");
    kll::emitters::kiibohd::write(&outfile, &kll_state);
}
