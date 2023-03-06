// 시험(Fail)
use std::{cmp::Ordering, io::stdin};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Score {
    p: u32,
    q: u32,
}

#[derive(Eq, PartialEq)]
struct Rate<'a> {
    i: usize,
    score: &'a Score,
}

impl<'a> PartialOrd for Rate<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rate_self = self.score.p as f64 / self.score.q as f64;
        let rate_other = other.score.p as f64 / other.score.q as f64;
        match rate_self.partial_cmp(&rate_other) {
            Some(Ordering::Equal) => {
                match self
                    .score
                    .q
                    .partial_cmp(&other.score.q)
                    .map(Ordering::reverse)
                {
                    Some(Ordering::Equal) => self.i.partial_cmp(&other.i),
                    o @ _ => o,
                }
            }
            o @ _ => o,
        }
    }
}

impl<'a> Ord for Rate<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Eq, PartialEq)]
struct Diff<'a> {
    i: usize,
    score: &'a Score,
}

impl<'a> PartialOrd for Diff<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.score.q - self.score.p)
            .partial_cmp(&(other.score.q - other.score.p))
            .map(Ordering::reverse)
        {
            Some(Ordering::Equal) => match self
                .score
                .q
                .partial_cmp(&other.score.q)
                .map(Ordering::reverse)
            {
                Some(Ordering::Equal) => self.i.partial_cmp(&other.i),
                o @ _ => o,
            },
            o @ _ => o,
        }
    }
}

impl<'a> Ord for Diff<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Eq, PartialEq)]
struct Add<'a> {
    i: usize,
    score: &'a Score,
}

impl<'a> PartialOrd for Add<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.score.q + self.score.p)
            .partial_cmp(&(other.score.q + other.score.p))
            .map(Ordering::reverse)
        {
            Some(Ordering::Equal) => match self
                .score
                .q
                .partial_cmp(&other.score.q)
                .map(Ordering::reverse)
            {
                Some(Ordering::Equal) => self.i.partial_cmp(&other.i),
                o @ _ => o,
            },
            o @ _ => o,
        }
    }
}

impl<'a> Ord for Add<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Eq, PartialEq)]
struct Mul<'a> {
    i: usize,
    score: &'a Score,
}

impl<'a> PartialOrd for Mul<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.score.q as u64 * self.score.p as u64)
            .partial_cmp(&(other.score.q as u64 * other.score.p as u64))
            .map(Ordering::reverse)
        {
            Some(Ordering::Equal) => match self
                .score
                .q
                .partial_cmp(&other.score.q)
                .map(Ordering::reverse)
            {
                Some(Ordering::Equal) => self.i.partial_cmp(&other.i),
                o @ _ => o,
            },
            o @ _ => o,
        }
    }
}

impl<'a> Ord for Mul<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let mut lines = stdin().lines();
    let input = lines.next().unwrap().unwrap();
    let mut vars = input.split_whitespace();
    let n = vars.next().unwrap().parse().unwrap();
    let k: u32 = vars.next().unwrap().parse().unwrap();

    let mut scores = Vec::with_capacity(n);
    for _ in 0..n {
        let input = lines.next().unwrap().unwrap();
        let mut exam = input.split_whitespace();
        let score = Score {
            p: exam.next().unwrap().parse().unwrap(),
            q: exam.next().unwrap().parse().unwrap(),
        };
        scores.push(score);
    }

    let mut rates = Vec::with_capacity(n);
    let mut diffs = Vec::with_capacity(n);
    let mut adds = Vec::with_capacity(n);
    let mut muls = Vec::with_capacity(n);
    for (i, score) in scores.iter().enumerate() {
        rates.push(Rate { i, score });
        diffs.push(Diff { i, score });
        adds.push(Add { i, score });
        muls.push(Mul { i, score });
    }
    rates.sort();
    diffs.sort();
    adds.sort();
    muls.sort();

    let mut p_accum = 0u64;
    let mut q_accum = 0u64;
    for _ in 0..k {
        let rate = rates.last().unwrap();
        let diff = diffs.last().unwrap();
        let add = adds.last().unwrap();
        let mul = muls.last().unwrap();
        let if_rate =
            (p_accum as f64 + rate.score.p as f64) / (q_accum as f64 + rate.score.q as f64);
        let if_diff =
            (p_accum as f64 + diff.score.p as f64) / (q_accum as f64 + diff.score.q as f64);
        let if_add = (p_accum as f64 + add.score.p as f64) / (q_accum as f64 + add.score.q as f64);
        let if_mul = (p_accum as f64 + mul.score.p as f64) / (q_accum as f64 + mul.score.q as f64);

        let (i, score) = if if_rate >= if_diff && if_rate >= if_add && if_rate >= if_mul {
            println!("!rate {} {}", rate.score.p, rate.score.q);
            (rate.i, rate.score)
        } else if if_diff > if_rate && if_diff > if_add && if_diff > if_mul {
            println!("!diff {} {}", diff.score.p, diff.score.q);
            (diff.i, diff.score)
        } else if if_add > if_rate && if_add > if_diff && if_add > if_mul {
            println!("!add_ {} {}", add.score.p, add.score.q);
            (add.i, add.score)
        } else {
            println!("!mul_ {} {}", mul.score.p, mul.score.q);
            (mul.i, mul.score)
        };
        p_accum += score.p as u64;
        q_accum += score.q as u64;
        rates.remove(rates.binary_search(&Rate { i, score }).unwrap());
        diffs.remove(diffs.binary_search(&Diff { i, score }).unwrap());
        adds.remove(adds.binary_search(&Add { i, score }).unwrap());
        muls.remove(muls.binary_search(&Mul { i, score }).unwrap());
    }
    println!("{}", p_accum as f64 / q_accum as f64);
}
