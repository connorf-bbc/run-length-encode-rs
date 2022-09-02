mod chunk;
mod encode;
mod shade;

use crate::chunk::runs;
use crate::shade::Shade::{Greyscale, Mono};
use std::env;
use std::fs;

fn help() {
    println!("usage:
rle mono <inputfile>
    monochrome image encoding
rle grey <inputfile>
    greyscale (hex shade) image encoding");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let shade = match &(&args[1])[..] {
                "mono" => Mono,
                "grey" => Greyscale,
                _ => {
                    eprintln!("error: first argument not mono or grey");
                    help();
                    return;
                }
            };

            let in_file_path = &args[2];

            let contents =
                fs::read_to_string(in_file_path).expect("could not read input file");

            println!("{}", runs(&contents, shade));
        }
        _ => {
            help();
        }
    }
}
