use std::env;

use crate::modules::{array, controle};

mod modules;

const REQUIRD_ARGS: usize = 2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args: Vec<String> = args
        .into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // println!("args = {:#?}", args);

    if args.len() < REQUIRD_ARGS {
        panic!("the lenght of args must greater or equal to 1");
    }

    let nom: &String = &args[1];

    match nom.as_str() {
        "var" => modules::var::var(),
        "controle" => controle::controle(),
        "array" => array::array(),
        "slice" => modules::slice::slice(),
        "struct" => modules::struct_::struct_(),
        "enum" => modules::enum_::enum_(),
        _ => println!("Unkown name : {}", nom),
    }
}
