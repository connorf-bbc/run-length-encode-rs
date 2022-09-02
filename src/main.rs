fn main() {
    println!("Hello, world!");
}

fn runs(s: &str) -> String {
    return chunks(s)
        .iter()
        .map(|(_c, n)| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
}

fn chunks(s: &str) -> Vec<(char, usize)> {
    let mut acc = Vec::<(char, usize)>::with_capacity(s.len());

    s.chars().for_each(|c| match acc.pop() {
        None => {
            if c == '1' {
                // b/w encoding always starts from black
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
    assert_eq!(runs(""), "");
}

#[test]
fn one_black() {
    assert_eq!(runs("0"), "1");
}

#[test]
fn two_black() {
    assert_eq!(runs("00"), "2");
}

#[test]
fn one_white() {
    assert_eq!(runs("1"), "0,1");
}

#[test]
fn black_then_white() {
    assert_eq!(runs("01"), "1,1");
}

#[test]
fn white_then_black() {
    assert_eq!(runs("10"), "0,1,1");
}

#[test]
fn blacks_then_whites() {
    assert_eq!(runs("00111"), "2,3");
}

#[test]
fn whites_then_blacks() {
    assert_eq!(runs("11100"), "0,3,2");
}

#[test]
fn blacks_then_whites_then_blacks() {
    assert_eq!(runs("0011100000"), "2,3,5");
}
