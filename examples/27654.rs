// 시험(Fail)
use std::{cmp::Ordering, collections::BinaryHeap, io::stdin};

#[derive(Eq, Ord)]
struct Score(u32, u32);

impl PartialEq<Self> for Score {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rate_self = self.0 as f64 / self.1 as f64;
        let rate_other = other.0 as f64 / other.1 as f64;
        if (rate_self - rate_other).abs() < f64::EPSILON {
            if self.1 > other.1 {
                Some(Ordering::Greater)
            } else if self.1 < other.1 {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Equal)
            }
        } else if rate_self > rate_other {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

fn main() {
    let mut lines = stdin().lines();
    let input = lines.next().unwrap().unwrap();
    let mut vars = input.split_whitespace();
    let n = vars.next().unwrap().parse().unwrap();
    let k = vars.next().unwrap().parse().unwrap();

    let mut max_heap = BinaryHeap::with_capacity(n);
    for _ in 0..n {
        let input = lines.next().unwrap().unwrap();
        let mut exam = input.split_whitespace();
        let p: u32 = exam.next().unwrap().parse().unwrap();
        let q: u32 = exam.next().unwrap().parse().unwrap();

        max_heap.push(Score(p, q));
    }

    let mut p_accum = 0;
    let mut q_accum = 0;
    for _ in 0..k {
        if let Some(Score(p, q)) = max_heap.pop() {
            p_accum += p;
            q_accum += q;
        }
    }
    println!("{}", p_accum as f64 / q_accum as f64);
}