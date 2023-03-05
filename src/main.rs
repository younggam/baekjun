use std::io::stdin;
use std::time::Instant;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Score {
    p: u32,
    q: u32,
}

fn main() {
    let mut lines = stdin().lines();
    let input = lines.next().unwrap().unwrap();
    let mut vars = input.split_whitespace();
    let n: usize = vars.next().unwrap().parse().unwrap();
    let k: u32 = vars.next().unwrap().parse().unwrap();

    let mut first = 0;
    let mut scores = Vec::with_capacity(n);
    for i in 0..n {
        let input = lines.next().unwrap().unwrap();
        let mut exam = input.split_whitespace();
        let score = Score {
            p: exam.next().unwrap().parse().unwrap(),
            q: exam.next().unwrap().parse().unwrap(),
        };
        scores.push(score);
        let score = &scores[i];
        let rate = score.p as f64 / score.q as f64;
        let first_score = &scores[first];
        let first_rate = first_score.p as f64 / first_score.q as f64;
        if first_rate == rate {
            if score.q > first_score.q {
                first = i;
            }
        } else if first_rate < rate {
            first = i;
        }
    }
    let time = Instant::now();

    let first = scores.swap_remove(first);
    println!("! {} {}", first.p, first.q);
    let mut p_accum = first.p as u64;
    let mut q_accum = first.q as u64;
    for _ in 1..k {
        let mut i = 0;
        for j in 1..scores.len() {
            let cur = &scores[i];
            let score = &scores[j];
            let rate_with_score =
                (p_accum as f64 + score.p as f64) / (q_accum as f64 + score.q as f64);
            let rate_with_cur = (p_accum as f64 + cur.p as f64) / (q_accum as f64 + cur.q as f64);
            if rate_with_cur < rate_with_score {
                i = j;
            } else if rate_with_cur == rate_with_score && cur.q > score.q {
                i = j;
            }
        }
        let score = scores.swap_remove(i);
        println!("! {} {}", score.p, score.q);

        p_accum += score.p as u64;
        q_accum += score.q as u64;
    }
    println!(
        "{} {}",
        p_accum as f64 / q_accum as f64,
        time.elapsed().as_secs_f64()
    );
}
