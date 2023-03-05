// 증가 배열 만들기
use std::io::{stdin, stdout, Write};

fn main() {
    let mut lines = stdin().lines();
    let input = lines.next().unwrap().unwrap();
    let mut vars = input.split_whitespace();
    let n: usize = vars.next().unwrap().parse().unwrap();
    let m: usize = vars.next().unwrap().parse().unwrap();
    let k: usize = vars.next().unwrap().parse().unwrap();

    if n + m - 1 > k {
        println!("NO");
    } else {
        let mut stdout = stdout().lock();
        let _ = writeln!(stdout, "YES");
        for i in 1..=n {
            let mut j = 0;
            while j < m - 1 {
                let _ = write!(stdout, "{} ", i + j);
                j += 1;
            }
            let _ = writeln!(stdout, "{}", i + j);
        }
    }
}
