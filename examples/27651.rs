// 벌레컷
use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut total = 0u64;
    let mut accums = Vec::with_capacity(n);
    for part in lines.next().unwrap().unwrap().split_whitespace() {
        let part: u64 = part.parse().unwrap();
        total += part;
        accums.push(total);
    }

    let mut answer = 0u64;
    let mut prev_belly = 0;
    let mut belly_i = accums.len() - 1;
    let mut prev_chest = accums[0];
    let mut chest_i = 1;
    for i in 0..accums.len() {
        let head = accums[i];
        let mut broke = false;
        let mut belly = prev_belly;
        for j in (i + 2..=belly_i).rev() {
            let part = accums[j] - accums[j - 1];
            belly += part;
            if head < belly {
                prev_belly = belly - part;
                belly_i = j;
                broke = true;
                break;
            }
        }
        if !broke {
            break;
        }
        broke = false;
        let mut chest = prev_chest - accums[i];
        let mut rest = total - head - chest;
        for j in chest_i..belly_i {
            let part = accums[j] - accums[j - 1];
            chest += part;
            rest -= part;
            if chest > rest && rest > head {
                prev_chest = accums[j - 1];
                chest_i = j;
                broke = true;
                break;
            }
        }
        if !broke {
            break;
        }
        answer += (belly_i - chest_i) as u64;
    }
    println!("{answer}");
}
