#[allow(dead_code,non_upper_case_globals)]
mod kll_defines {
    include!(concat!(env!("OUT_DIR"), "/kll_defines.rs"));
}

fn main() {
    println!("myDefine: {}", kll_defines::myDefine);
    println!("myIntDefine: {}", kll_defines::myIntDefine);
}
