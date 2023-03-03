// Toboggan Ride
// use std::io::stdin;
//
// const MAX_ERROR: u32 = 1 << 18;
//
// fn main() {
//     let mut lines = stdin().lines();
//     let input = lines.next().unwrap().unwrap();
//     let mut vars = input.split_whitespace();
//     let l: u32 = vars.next().unwrap().parse().unwrap();
//     let n: usize = vars.next().unwrap().parse().unwrap();
//     let t: f64 = vars.next().unwrap().parse().unwrap();
//
//     let input = lines.next().unwrap().unwrap();
//     let mut distances = [0; 100];
//     let mut prev_b = 0;
//     let mut max_distance = 0;
//     let mut i = 0;
//     for b in input.split_whitespace() {
//         let b: u32 = b.parse().unwrap();
//         if b == 0 { continue; }
//         let distance = b - prev_b;
//         prev_b = b;
//         max_distance = max_distance.max(distance);
//         distances[i] = distance;
//         i += 1;
//     }
//     let distance = l - prev_b;
//     max_distance = max_distance.max(distance);
//     distances[i] = distance;
//
//     let pivot_boost = (2. * max_distance as f64).sqrt();
//     let mut boost = pivot_boost;
//     let mut step = 1u32;
//     let mut once_less = false;
//     loop {
//         let mut time = 0.;
//         let mut velocity = 0.;
//         let mut broke = false;
//         for i in 0..n {
//             let distance = distances[i] as f64;
//             velocity += boost;
//             let interval = velocity - (velocity * velocity - 2. * distance).sqrt();
//             if interval.is_nan() || interval < 0. {
//                 broke = true;
//                 break;
//             }
//             time += interval;
//             velocity -= interval;
//         }
//
//         if step > MAX_ERROR {
//             break;
//         }
//
//         if time > t || broke {
//             if once_less {
//                 step <<= 1;
//             }
//             boost += pivot_boost / step as f64;
//         } else if time < t {
//             step <<= 1;
//             boost -= pivot_boost / step as f64;
//             once_less = true;
//         } else {
//             break;
//         }
//     }
//     println!("{boost}");
// }