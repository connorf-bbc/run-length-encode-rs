fn main() {
    println!("Hello, world!");
}

fn runs_basic(s: &str) -> String {
    if s.is_empty() {
        return "".to_string();
    }

    let n_black = s.chars().filter(|c| *c == '0').count();
    let n_white = s.chars().filter(|c| *c == '1').count();

    if n_white == 0 {
        return n_black.to_string();
    } else {
        return format!("{:?},{:?}", n_black, n_white);
    }
}

fn runs_foreach(s: &str) -> String {
    let mut current_run: Option<(char, usize)> = None;
    let mut acc: String = String::with_capacity(10);

    s.chars().for_each(|c| match current_run {
        None => {
            if c == '1' {
                acc.push_str("0,");
            }
            current_run = Some((c, 1));
        }
        Some((run_c, n_run_c)) => {
            if c == run_c {
                current_run = Some((run_c, n_run_c + 1));
            } else {
                acc.push_str(&n_run_c.to_string());
                acc.push(',');
                current_run = Some((c, 1));
            }
        }
    });

    match current_run {
        None => {}
        Some((_run_c, n_run_c)) => {
            acc.push_str(&n_run_c.to_string());
        }
    }

    return acc;
}

fn runs_fold(s: &str) -> String {
    return chunks(s)
        .iter()
        .map(|(_c, n)| n.to_string())
        .collect();
}

// fold s into an list of runs (char, length)
fn chunks(s: &str) -> Vec<(char, usize)> {
    // can we push and pop off the end of vec instead of needing this?
    // will that let us prevent the off by one issue
    let mut current_run: Option<(char, usize)> = None;

    return s
        .chars()
        .fold(
            Vec::<(char, usize)>::with_capacity(s.len()),
            |mut acc, c| {
                match current_run {
                    None => {
                        if c == '1' {
                            // b/w encoding always starts from black
                            acc.push(('0', 0));
                        }
                        current_run = Some((c, 1));
                    }
                    Some((run_c, n_run_c)) => {
                        if c == run_c {
                            current_run = Some((run_c, n_run_c + 1));
                        } else {
                            acc.push((run_c, n_run_c));
                            current_run = Some((c, 1));
                        }
                    }
                }
                acc
            },
        );
}

fn runs(s: &str) -> String {
    return runs_fold(s);
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
