// Parentheses Order (Large)
use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut dp = [[0u64; 102]; 102];
    dp[1][0] = 1;
    for i in 1..=101 {
        for j in i..=101 {
            dp[i][j] = dp[i][j - 1].saturating_add(dp[i - 1][j]);
        }
    }

    for i in 1..=t {
        let input = lines.next().unwrap().unwrap();
        let mut vars = input.split_whitespace();
        let n: usize = vars.next().unwrap().parse().unwrap();
        let mut k: u64 = vars.next().unwrap().parse().unwrap();

        let mut x = n;
        let mut y = n + 1;
        if k > dp[x][y] {
            println!("Case #{i}: Doesn't Exist!");
            continue;
        }

        let mut answer = String::with_capacity(2 * n);
        while y > 1 {
            if k <= dp[x][y] {
                answer.push('(');
                x -= 1;
            } else {
                answer.push(')');
                k -= dp[x][y];
                y -= 1;
            }
        }
        println!("Case #{i}: {answer}");
    }
}