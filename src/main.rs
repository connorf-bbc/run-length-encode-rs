mod shade;
mod encode;
mod chunk;

use crate::shade::Shade::{Mono, Greyscale};
use crate::chunk::runs;
use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let shade = match &(&args[1])[..] {
        "m" => Mono,
        "g" => Greyscale,
        _ => panic!("usage: [m|g] inputfile")
    };
    let in_file_path = &args[2];

    let contents = fs::read_to_string(in_file_path).expect("Should be able to read input file");

    println!("{}", runs(&contents, shade));
}

