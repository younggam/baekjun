use std::{io::*, num::ParseIntError};

enum Dir {
    Left,
    DOWN,
    RIGHT,
    UP,
}

impl Dir {
    fn next(self) -> Dir {
        match self {
            Dir::Left => Dir::DOWN,
            Dir::DOWN => Dir::RIGHT,
            Dir::RIGHT => Dir::UP,
            Dir::UP => Dir::Left,
        }
    }

    fn rot(&self, r: isize, c: isize) -> (isize, isize) {
        match self {
            Dir::Left => (r, c),
            Dir::DOWN => (-c, r),
            Dir::RIGHT => (-r, -c),
            Dir::UP => (c, -r),
        }
    }

    fn as_delta(&self) -> (isize, isize) {
        match self {
            Dir::Left => (0, -1),
            Dir::DOWN => (1, 0),
            Dir::RIGHT => (0, 1),
            Dir::UP => (-1, 0),
        }
    }
}

const MAX_N: usize = 499;
const BLOW_TABLE: [(isize, isize, u16); 9] = [
    (-2, 0, 2),
    (-1, -1, 10),
    (-1, 0, 7),
    (-1, 1, 1),
    (0, -2, 5),
    (1, -1, 10),
    (1, 0, 7),
    (1, 1, 1),
    (2, 0, 2),
];

fn main() -> std::result::Result<(), ParseIntError> {
    let mut a = [[0u16; MAX_N]; MAX_N];

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let n: usize = input.next().unwrap().parse()?;

    for r in 0..n {
        for c in 0..n {
            a[r][c] = input.next().unwrap().parse()?;
        }
    }

    let mut i = 0u16;
    let mut k = 1u16;
    let mut dir = Dir::Left;
    let mut cur = (n as isize / 2, n as isize / 2);
    let mut answer = 0u32;
    for _ in 1..n * n {
        let (dr, dc) = dir.as_delta();
        cur.0 += dr;
        cur.1 += dc;

        let sand = a[cur.0 as usize][cur.1 as usize];
        a[cur.0 as usize][cur.1 as usize] = 0;
        let mut total_blown = 0u16;
        for (mut r, mut c, rate) in BLOW_TABLE {
            let blown = sand * rate / 100;
            total_blown += blown;
            (r, c) = dir.rot(r, c);
            r += cur.0;
            c += cur.1;
            if r < 0 || r >= n as isize || c < 0 || c >= n as isize {
                answer += blown as u32;
            } else {
                a[r as usize][c as usize] += blown;
            }
        }
        let (mut r, mut c) = dir.rot(0, -1);
        r += cur.0;
        c += cur.1;
        if r < 0 || r >= n as isize || c < 0 || c >= n as isize {
            answer += (sand - total_blown) as u32;
        } else {
            a[r as usize][c as usize] += sand - total_blown;
        }

        i += 1;
        if i >= 2 * k {
            i = 0;
            k += 1;
            dir = dir.next();
        } else if i == k {
            dir = dir.next();
        }
    }

    println!("{answer}");

    Ok(())
}
