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
    greyscale (hex shade) image encoding
rle grey n <inputfile>
    greyscale (hex shade) image encoding with a compression factor (0 to 15)");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let shade = match &(&args[1])[..] {
                "mono" => Mono,
                "grey" => Greyscale(0),
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
        4 => {
            if &args[1] != "grey" {
                help();
                return;
            }

            let compression: u8 = match args[2].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("could not parse compression factor");
                    help();
                    return;
                }
            };

            let in_file_path = &args[3];

            let contents =
                fs::read_to_string(in_file_path).expect("could not read input file");

            println!("{}", runs(&contents, Greyscale(compression)));
        }
        _ => {
            help();
        }
    }
}
