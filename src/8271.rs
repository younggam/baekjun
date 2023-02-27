// use std::{collections::VecDeque, io::{stdin, stdout, Write}};
//
// const W: u8 = 87;
//
// fn main() {
//     let mut lines = stdin().lines();
//     let n = lines.next().unwrap().unwrap().parse().unwrap();
//
//     let mut rook_map = Vec::with_capacity(n);
//     let mut rows = VecDeque::with_capacity(n);
//     let mut columns = VecDeque::with_capacity(n);
//     for i in 0..n {
//         rook_map.push(lines.next().unwrap().unwrap());
//         rows.push_back(i as u16);
//         columns.push_back(i as u16);
//     }
//
//     while let Some(&y) = rows.front() {
//         let y = y as usize;
//         let mut i = 0usize;
//         'x: while i < columns.len() {
//             let x = *columns.get(i).unwrap() as usize;
//             for (index, &l) in rows.iter().enumerate() {
//                 if rook_map[l as usize].as_bytes()[x] == W {
//                     columns.remove(i);
//                     rows.remove(index);
//                     if i == 0 {
//                         break 'x;
//                     } else {
//                         i += 1;
//                         continue 'x;
//                     }
//                 }
//             }
//             for (index, &l) in columns.iter().enumerate() {
//                 if rook_map[y].as_bytes()[l as usize] == W {
//                     columns.remove(index);
//                     rows.pop_front();
//                     break 'x;
//                 }
//             }
//             rook_map[y].replace_range(x..=x, "W");
//             columns.remove(i);
//             rows.pop_front();
//             break;
//         }
//     }
//
//     let mut stdout = stdout().lock();
//     for rook_line in rook_map {
//         writeln!(stdout, "{rook_line}").unwrap();
//     }
// }