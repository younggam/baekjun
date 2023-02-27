use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let t: u8 = lines.next().unwrap().unwrap().parse().unwrap();

    for i in 1..=t {
        let input = lines.next().unwrap().unwrap();
        let mut vars = input.split_whitespace();
        let n: u8 = vars.next().unwrap().parse().unwrap();
        let mut k: u64 = vars.next().unwrap().parse().unwrap();

        let mut catalans = vec![0; n as usize + 1];
        catalans[0] = 1u64;
        for j in 1..(n as usize + 1) {
            for l in 0..j {
                catalans[j] += catalans[l] * catalans[j - l - 1];
            }
        }

        if catalans[n as usize] < k {
            println!("Case #{i}: Doesn't Exist!");
            continue;
        }

        let mut answer = String::with_capacity(n as usize * 2);
        answer.push('(');
        let mut open = n - 1;
        let mut catalan0 = catalans[n as usize];
        let mut catalan_i = n as usize - 1;
        for j in 0..n * 2 - 1 {
            let catalan_diff = if catalan_i == 0 {
                catalan0
            } else if catalan0 > catalans[catalan_i] {
                catalan0 - catalans[catalan_i]
            } else if catalan0 <= (n - open + 1) as u64 && catalan0 > 1 {
                catalan_i = 0;
                catalan0 = 1;
                1
            } else {
                catalan_i = catalan_i.saturating_sub(1);
                catalan0 - catalans[catalan_i]
            };
            if open > 0 && catalan_diff >= k {
                answer.push('(');
                open -= 1;
                catalan0 = if catalan_i == 0 { 0 } else { catalan_diff };
            } else {
                answer.push(')');
                if open > 0 {
                    k -= catalan_diff;
                }
            }
        }
        println!("Case #{i}: {answer}");
        // fn find_valid_parentheses(n: u8, k: u64, j: &mut u64, val0: u128, val1: u128, open: u8, close: u8) -> Option<(u128, u128)> {
        //     if open == n && close == n {
        //         *j += 1;
        //         return if *j == k {
        //             Some((val0, val1))
        //         } else {
        //             None
        //         };
        //     }
        //
        //     let mut ret = None;
        //     if open < n {
        //         let l = open + close;
        //         let mut val0 = val0;
        //         let mut val1 = val1;
        //         if l >= 128 {
        //             val0 |= 1 << (l - 128);
        //         } else {
        //             val1 |= 1 << l;
        //         }
        //         ret = find_valid_parentheses(n, k, j, val0, val1, open + 1, close);
        //     }
        //
        //     if close < open {
        //         ret = ret.or(find_valid_parentheses(n, k, j, val0, val1, open, close + 1));
        //     }
        //
        //     ret
        // }
        //
        // let mut j = 0;
        // match find_valid_parentheses(n, k, &mut j, 0, 0, 0, 0) {
        //     Some((mut val0, mut val1)) => {
        //         let mut valid = String::with_capacity((2 * n) as usize);
        //         for _ in 0..2 * n {
        //             if val1 & 1 == 0 {
        //                 valid.push(')');
        //             } else {
        //                 valid.push('(');
        //             }
        //
        //             val1 >>= 1;
        //             if val0 & 1 == 1 {
        //                 val1 += 0x1000_0000_0000_0000_0000_0000_0000_0000;
        //             }
        //             val0 >>= 1;
        //         }
        //         println!("Case #{i}: {}", valid);
        //     }
        //     None => println!("Case #{i}: Doesn't Exist!")
        // };
    }
}