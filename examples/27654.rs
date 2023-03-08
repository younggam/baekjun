// 시험
use std::io::stdin;

const MAX_ERROR: f64 = 0.000001;

#[derive(Copy, Clone)]
struct Score {
    p: u32,
    q: u32,
}

impl Score {
    fn as_line_equation(&self, x: f64) -> f64 {
        -(self.q as f64) * x + self.p as f64
    }
}

fn main() {
    let mut lines = stdin().lines();
    let input = lines.next().unwrap().unwrap();
    let mut vars = input.split_whitespace();
    let n: usize = vars.next().unwrap().parse().unwrap();
    let k: u32 = vars.next().unwrap().parse().unwrap();

    let mut scores = [Score { p: 0, q: 0 }; 100000];
    for i in 0..n {
        let input = lines.next().unwrap().unwrap();
        let mut exam = input.split_whitespace();
        scores[i].p = exam.next().unwrap().parse().unwrap();
        scores[i].q = exam.next().unwrap().parse().unwrap();
    }
    let scores_slice = &mut scores[0..n];

    let mut min = 0.;
    let mut max = 1.;
    loop {
        scores_slice.sort_by(|a: &Score, b: &Score| {
            let pivot = (min + max) * 0.5;
            a.as_line_equation(pivot)
                .partial_cmp(&b.as_line_equation(pivot))
                .unwrap()
        });
        let mut scores_iter = scores_slice.iter();
        let mut p_accum = 0u64;
        let mut q_accum = 0u64;
        for _ in 0..k {
            let score = scores_iter.next_back().unwrap();
            p_accum += score.p as u64;
            q_accum += score.q as u64;
        }
        let average = p_accum as f64 / q_accum as f64;
        let pivot = (min + max) * 0.5;
        let diff = (average - pivot).abs();
        if diff <= MAX_ERROR || diff / pivot <= MAX_ERROR {
            println!("{}", average);
            break;
        } else if average >= pivot {
            min = pivot;
        } else {
            max = pivot;
        }
    }
}
