use std::fs;

fn main() {
    let file = fs::read_to_string("test.kll").expect("cannot read file");
    let kll = kll_rs::parse(&file).unwrap();
    //println!("{:#?}", kll);
    println!("{}", kll);
}
