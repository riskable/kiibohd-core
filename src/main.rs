#[allow(dead_code,non_upper_case_globals)]
mod kll_defines {
    include!(concat!(env!("OUT_DIR"), "/kll_defines.rs"));
}

fn main() {
    println!("myCDefine: {}", kll_defines::myCDefine);
    println!("myIntDefine: {}", kll_defines::myIntDefine);
}
