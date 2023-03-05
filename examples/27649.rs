// 토크나이저
use std::io::{stdin, stdout, Write};

fn main() {
    let mut lines = stdin().lines();
    let input = lines.next().unwrap().unwrap();
    let split_whitespace = input.split_whitespace();
    let mut stdout = stdout().lock();
    for split in split_whitespace {
        let mut chars = split.chars();
        let mut separator = false;
        let mut first = true;
        while let Some(char) = chars.next() {
            match char {
                '<' | '>' | '(' | ')' => {
                    if !first || separator {
                        let _ = write!(stdout, " ");
                    }
                    let _ = write!(stdout, "{char}");
                    separator = true;
                }
                '|' | '&' => {
                    if !first || separator {
                        let _ = write!(stdout, " ");
                    }
                    let _ = write!(stdout, "{char}");
                    let _ = chars.next();
                    let _ = write!(stdout, "{char}");
                    separator = true;
                }
                _ => {
                    if separator {
                        let _ = write!(stdout, " ");
                        separator = false;
                    }
                    let _ = write!(stdout, "{char}");
                }
            }
            first = false;
        }
        let _ = write!(stdout, " ");
    }
}
