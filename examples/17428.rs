// K번째 괄호 문자열
use std::io::{stdin, stdout, Write};

fn main() {
    let mut lines = stdin().lines();
    let input = lines.next().unwrap().unwrap();
    let mut vars = input.split_whitespace();
    let n: usize = vars.next().unwrap().parse().unwrap();
    let k: u64 = vars.next().unwrap().parse().unwrap();
    let n = n / 2;
    let mut k = k + 1;

    let mut dp = vec![vec![0; n + 2]; n + 2];
    dp[1][0] = 1;
    for i in 1..=n + 1 {
        for j in i..=n + 1 {
            dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
        }
    }
    let mut x = n;
    let mut y = n + 1;
    if k > dp[x][y] {
        print!("-1");
        return;
    }

    let mut stdout = stdout().lock();
    while y > 1 {
        if k <= dp[x][y] {
            let _ = write!(stdout, "(");
            x -= 1;
        } else {
            let _ = write!(stdout, ")");
            k -= dp[x][y];
            y -= 1;
        }
    }
}