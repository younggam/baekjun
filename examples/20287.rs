// Lavaspar
use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lines();
    let vars = lines.next().unwrap().unwrap();
    let mut vars = vars.split_whitespace();

    let l: usize = vars.next().unwrap().parse().unwrap();
    let c: usize = vars.next().unwrap().parse().unwrap();

    let mut word_map_raw = Vec::with_capacity(l);
    let mut word_map = Vec::with_capacity(l);
    for i in 0..l {
        word_map_raw.push(lines.next().unwrap().unwrap());
        word_map.push(word_map_raw[i].chars().collect::<Vec<_>>());
    }

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        words.push(lines.next().unwrap().unwrap());
    }

    let mut specials = vec![0u32; c * l];
    for (k, word) in words.iter().enumerate() {
        let word_len = word.len();
        let max_down = match l.checked_sub(word_len) {
            Some(v) => v as isize,
            None => -1,
        };
        let max_right = match c.checked_sub(word_len) {
            Some(v) => v as isize,
            None => -1,
        };
        for i in 0..l {
            for j in 0..c {
                let mut right = (1 << word_len) - 1;
                let mut diag_up = right;
                let mut diag_down = right;
                let mut down = right;
                for m in 0..word_len {
                    if j as isize <= max_right {
                        for (index, spell) in word.chars().enumerate() {
                            if right & (1 << index) > 0 && spell == word_map[i][j + m] {
                                right ^= 1 << index;
                                break;
                            }
                        }
                        if i >= word_len - 1 {
                            for (index, spell) in word.chars().enumerate() {
                                if diag_up & (1 << index) > 0 && spell == word_map[i - m][j + m] {
                                    diag_up ^= 1 << index;
                                    break;
                                }
                            }
                        }
                        if i as isize <= max_down {
                            for (index, spell) in word.chars().enumerate() {
                                if diag_down & (1 << index) > 0 && spell == word_map[i + m][j + m] {
                                    diag_down ^= 1 << index;
                                    break;
                                }
                            }
                        }
                    }
                    if i as isize <= max_down {
                        for (index, spell) in word.chars().enumerate() {
                            if down & (1 << index) > 0 && spell == word_map[i + m][j] {
                                down ^= 1 << index;
                                break;
                            }
                        }
                    }
                }
                for m in 0..word_len {
                    if right == 0 { specials[i * c + j + m] |= 1 << k; }
                    if diag_up == 0 { specials[(i - m) * c + j + m] |= 1 << k; }
                    if diag_down == 0 { specials[(i + m) * c + j + m] |= 1 << k; }
                    if down == 0 { specials[(i + m) * c + j] |= 1 << k; }
                }
            }
        }
    }

    let mut answer = 0;
    for special in &specials {
        let mut count = 0;
        for i in 0..n {
            if special & (1 << i) != 0 { count += 1; }
        }
        if count > 1 { answer += 1; }
    }
    println!("{answer}");
}