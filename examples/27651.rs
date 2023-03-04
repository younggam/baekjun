// 벌레컷(Fail)
use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut total = 0;
    let mut parts = Vec::with_capacity(n);
    for part in lines.next().unwrap().unwrap().split_whitespace() {
        let part: u64 = part.parse().unwrap();
        total += part;
        parts.push(part);
    }

    let mut answer = 0u64;
    let mut prev_belly = 0;
    let mut belly_f_i = parts.len();
    let mut belly_l_i = parts.len();
    let mut prev_chest = 0;
    let mut chest_i = 1;
    let mut heads = Vec::with_capacity(n);
    for i in 0..parts.len() {
        let mut chest = total;
        let mut head = if i == 0 { 0 } else { heads[i - 1] };
        head += parts[i];
        heads.push(head);
        chest -= head;
        chest -= prev_belly;

        let mut belly = prev_belly;
        let prev_belly_i = belly_f_i;
        let mut belly_i_changed = false;
        for j in (chest_i + 1..belly_f_i).rev() {
            belly += parts[j];
            if head < belly && j < prev_belly_i {
                belly_i_changed = true;
                prev_belly = belly - parts[j];
                belly_f_i = j + 1;
                break;
            }
        }
        if head >= belly {
            break;
        }

        for j in (chest_i..belly_f_i) {}
    }
    println!("{answer}");
}