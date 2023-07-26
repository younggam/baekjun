use std::{io::*, mem::swap, num::ParseIntError};

const MAX_DEPTH: usize = 15;

fn main() -> std::result::Result<(), ParseIntError> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let n: usize = input.next().unwrap().parse()?;

    let mut depth = vec![0u16; n];
    let mut weights = vec![0; n];
    let mut parents = vec![[0u16; MAX_DEPTH + 1]; n];
    let mut graph = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        let u = input.next().unwrap().parse::<u16>()? - 1;
        let v = input.next().unwrap().parse::<u16>()? - 1;
        let weight: u16 = input.next().unwrap().parse()?;
        graph[u as usize].push((v, weight));
        graph[v as usize].push((u, weight));
    }

    let mut stack = Vec::with_capacity(n);
    stack.push((0, 0));
    while let Some((current, parent)) = stack.pop() {
        if current != 0 {
            depth[current] = depth[parent] + 1;
        }
        parents[current][0] = parent as u16;

        for i in 1..=MAX_DEPTH {
            let temp = parents[current][i - 1];
            parents[current][i] = parents[temp as usize][i - 1];
        }

        for child in &graph[current] {
            if child.0 != parent as u16 {
                weights[child.0 as usize] = weights[current] + child.1 as usize;
                stack.push((child.0 as usize, current))
            }
        }
    }

    let m: usize = input.next().unwrap().parse()?;
    let mut answers = Vec::with_capacity(m);

    for _ in 0..m {
        let mut u = input.next().unwrap().parse::<usize>()? - 1;
        let mut v = input.next().unwrap().parse::<usize>()? - 1;

        let mut weight = weights[u] + weights[v];
        if depth[u] != depth[v] {
            if depth[u] > depth[v] {
                swap(&mut u, &mut v);
            }
            for i in (0..=MAX_DEPTH).rev() {
                if depth[u] <= depth[parents[v][i] as usize] {
                    v = parents[v][i] as usize;
                }
            }
        }

        let mut lca = u;

        if u != v {
            for i in (0..=MAX_DEPTH).rev() {
                if parents[u][i] != parents[v][i] {
                    u = parents[u][i] as usize;
                    v = parents[v][i] as usize
                }
                lca = parents[u][i] as usize;
            }
        }

        weight -= 2 * weights[lca];
        answers.push(weight);
    }

    let mut stdout = BufWriter::new(stdout().lock());
    for answer in answers {
        writeln!(stdout, "{answer}").unwrap();
    }

    Ok(())
}
