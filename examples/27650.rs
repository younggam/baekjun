// 마법 박스
use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::with_capacity(n / 10);
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let mut left = 0;
    let mut mid = primes.len() / 2;
    let mut right = primes.len() - 1;
    loop {
        println!("? {}", primes[mid]);
        let is_yes = lines.next().unwrap().unwrap().parse::<u8>().unwrap() != 0;
        if is_yes {
            left = mid + 1;
        } else {
            right = mid;
        }
        mid = (left + right) / 2;
        if left == right {
            println!("! {}", primes[mid]);
            break;
        }
    }
}
