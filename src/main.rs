use std::{cmp::Ordering, io::stdin};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Score {
    p: u32,
    q: u32,
}

//
// #[derive(Eq)]
// struct Rate<'a> {
//     i: usize,
//     score: &'a Score,
// }
//
// impl<'a> PartialEq<Self> for Rate<'a> {
//     fn eq(&self, other: &Self) -> bool {
//         self.i == other.i && self.score.p == other.score.p && self.score.q == other.score.q
//     }
// }
//
// impl<'a> PartialOrd for Rate<'a> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         let rate_self = self.score.p as f64 / self.score.q as f64;
//         let rate_other = other.score.p as f64 / other.score.q as f64;
//         match rate_self.partial_cmp(&rate_other) {
//             Some(Ordering::Equal) => {
//                 match self.score.q.partial_cmp(&other.score.q).map(Ordering::reverse) {
//                     Some(Ordering::Equal) => {
//                         self.i.partial_cmp(&other.i)
//                     }
//                     o @ _ => o
//                 }
//             }
//             o @ _ => o
//         }
//     }
// }
//
// impl<'a> Ord for Rate<'a> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }
//
// #[derive(Eq)]
// struct Diff<'a> {
//     i: usize,
//     score: &'a Score,
// }
//
// impl<'a> PartialEq<Self> for Diff<'a> {
//     fn eq(&self, other: &Self) -> bool {
//         self.i == other.i && self.score.p == other.score.p && self.score.q == other.score.q
//     }
// }
//
// impl<'a> PartialOrd for Diff<'a> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match (self.score.q - self.score.p).partial_cmp(&(other.score.q - other.score.p)).map(Ordering::reverse) {
//             Some(Ordering::Equal) => {
//                 self.i.partial_cmp(&other.i)
//             }
//             o @ _ => o
//         }
//     }
// }
//
// impl<'a> Ord for Diff<'a> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }
//
// fn main() {
//     let mut lines = stdin().lines();
//     let input = lines.next().unwrap().unwrap();
//     let mut vars = input.split_whitespace();
//     let n = vars.next().unwrap().parse().unwrap();
//     let k = vars.next().unwrap().parse().unwrap();
//
//     let mut scores = Vec::with_capacity(n);
//     for _ in 0..n {
//         let input = lines.next().unwrap().unwrap();
//         let mut exam = input.split_whitespace();
//         let score = Score {
//             p: exam.next().unwrap().parse().unwrap(),
//             q: exam.next().unwrap().parse().unwrap(),
//         };
//         scores.push(score);
//     }
//
//     let mut rate_set = Vec::with_capacity(n);
//     let mut diff_set = Vec::with_capacity(n);
//     for (i, score) in scores.iter().enumerate() {
//         rate_set.push(Rate {
//             i,
//             score,
//         });
//         diff_set.push(Diff {
//             i,
//             score,
//         });
//     }
//     rate_set.sort();
//     diff_set.sort();
//
//     let mut p_accum = 0u64;
//     let mut q_accum = 0u64;
//     for _ in 0..k {
//         let rate = rate_set.last().unwrap();
//         let diff = diff_set.last().unwrap();
//         let if_rate = (p_accum as f64 + rate.score.p as f64) / (q_accum as f64 + rate.score.q as f64);
//         let if_diff = (p_accum as f64 + diff.score.p as f64) / (q_accum as f64 + diff.score.q as f64);
//         if if_rate >= if_diff {
//             let rate = rate_set.pop().unwrap();
//             p_accum += rate.score.p as u64;
//             q_accum += rate.score.q as u64;
//             diff_set.remove(diff_set.binary_search(&Diff {
//                 i: rate.i,
//                 score: rate.score,
//             }).unwrap());
//         } else if if_rate < if_diff {
//             let diff = diff_set.pop().unwrap();
//             p_accum += diff.score.p as u64;
//             q_accum += diff.score.q as u64;
//             rate_set.remove(rate_set.binary_search(&Rate {
//                 i: diff.i,
//                 score: diff.score,
//             }).unwrap());
//         }
//     }
//     println!("{}", p_accum as f64 / q_accum as f64);
// }
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
        let rate = score.p as f64 / score.q as f64;
        let score = Score {
            p: exam.next().unwrap().parse().unwrap(),
            q: exam.next().unwrap().parse().unwrap(),
        };
        scores.push(score);
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

    // let mut rate_set = Vec::with_capacity(n);
    // let mut diff_set = Vec::with_capacity(n);
    // for (i, score) in scores.iter().enumerate() {
    //     rate_set.push(Rate {
    //         i,
    //         score,
    //     });
    //     diff_set.push(Diff {
    //         i,
    //         score,
    //     });
    // }
    // rate_set.sort();
    // diff_set.sort();

    scores.remove(i);
    let mut p_accum = first.p as u64;
    let mut q_accum = first.q as u64;
    scores.sort_by(|a, b| {
        ((p_accum as f64 + a.p as f64) / (q_accum as f64 + a.q as f64))
            .partial_cmp(&((p_accum as f64 + a.p as f64) / (q_accum as f64 + a.q as f64)))
            .unwrap()
    });
    for _ in 1..k {
        for i in 0..scores.len() {
            let score = &scores[i];
            let test = (p_accum as f64 + score.p as f64) / (q_accum as f64 + score.q as f64);
            let if_diff =
                (p_accum as f64 + diff.score.p as f64) / (q_accum as f64 + diff.score.q as f64);
            if if_rate >= if_diff {
                let rate = rate_set.pop().unwrap();
                p_accum += rate.score.p as u64;
                q_accum += rate.score.q as u64;
                diff_set.remove(
                    diff_set
                        .binary_search(&Diff {
                            i: rate.i,
                            score: rate.score,
                        })
                        .unwrap(),
                );
            } else if if_rate < if_diff {
                let diff = diff_set.pop().unwrap();
                p_accum += diff.score.p as u64;
                q_accum += diff.score.q as u64;
                rate_set.remove(
                    rate_set
                        .binary_search(&Rate {
                            i: diff.i,
                            score: diff.score,
                        })
                        .unwrap(),
                );
            }
        }
    }
    println!("{}", p_accum as f64 / q_accum as f64);
}
