use crate::encode::*;
use crate::shade::Shade;
use crate::shade::Shade::{Greyscale, Mono};

pub type Chunk = (char, usize);

pub fn runs(s: &str, shade: Shade) -> String {
    let encoding: ChunkEncoding = match shade {
        Mono => chunk_encode_mono_hex,
        Greyscale => chunk_encode_greyscale_dec,
    };

    return chunks(s, shade)
        .iter()
        .map(encoding)
        .collect::<Vec<String>>()
        .join(",");
}

fn chunks(s: &str, shade: Shade) -> Vec<Chunk> {
    let mut acc = Vec::<Chunk>::with_capacity(s.len());

    s.chars()
        .filter(|c| !c.is_whitespace())
        .for_each(|c| match acc.pop() {
            None => {
                if c == '1' && shade == Mono {
                    // mono encoding always counts 0 first
                    acc.push(('0', 0));
                }
                acc.push((c, 1));
            }
            Some((run_c, n_run_c)) => {
                if c == run_c {
                    acc.push((run_c, n_run_c + 1));
                } else {
                    acc.push((run_c, n_run_c));
                    acc.push((c, 1));
                }
            }
        });
    return acc;
}

#[test]
fn empty() {
    assert_eq!(runs("", Mono), "");
}

// mono tests

#[test]
fn one_black() {
    assert_eq!(runs("0", Mono), "1");
}

#[test]
fn two_black() {
    assert_eq!(runs("00", Mono), "2");
}

#[test]
fn one_white() {
    assert_eq!(runs("1", Mono), "0,1");
}

#[test]
fn black_then_white() {
    assert_eq!(runs("01", Mono), "1,1");
}

#[test]
fn white_then_black() {
    assert_eq!(runs("10", Mono), "0,1,1");
}

#[test]
fn blacks_then_whites() {
    assert_eq!(runs("00111", Mono), "2,3");
}

#[test]
fn whites_then_blacks() {
    assert_eq!(runs("11100", Mono), "0,3,2");
}

#[test]
fn blacks_then_whites_then_blacks() {
    assert_eq!(runs("0011100000", Mono), "2,3,5");
}

// greyscale tests
#[test]
fn one_grey() {
    assert_eq!(runs("A", Greyscale), "A1");
}

#[test]
fn one_white_in_greyscale_does_not_count_black_first() {
    assert_eq!(runs("1", Greyscale), "11");
}
