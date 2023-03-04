use std::collections::BinaryHeap;
use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let (mut x_heap, mut y_heap) = (BinaryHeap::<i32>::with_capacity(n), BinaryHeap::<i32>::with_capacity(n));
    for _ in 0..n {
        let input = lines.next().unwrap().unwrap();
        let mut coords = input.split_whitespace();
        x_heap.push(coords.next().unwrap().parse().unwrap());
        y_heap.push(coords.next().unwrap().parse().unwrap());
    }

    let (mut prev_x, mut d_x, mut delta_x, mut prev_y, mut d_y, mut delta_y) = (0, 0, 0, 0, 0, 0);
    let mut x_heap_iter = x_heap.iter();
    if let Some(&first) = x_heap_iter.next() {
        prev_x = first;
    }
    for &x in x_heap_iter {
        delta_x = prev_x - x;
        d_x = d_x.max((delta_x).abs());
        println!("{prev_x}");
        prev_x = x;
    }
    let mut y_heap_iter = y_heap.iter();
    if let Some(&first) = y_heap_iter.next() {
        prev_y = first;
    }
    for &y in y_heap_iter {
        delta_y = prev_y - y;
        d_y = d_y.max((delta_y).abs());
        println!("{prev_y}");
        prev_y = y;
    }

    if d_x / delta_x.abs() > d_y / delta_y.abs() {

    } else if d_x / delta_x.abs() < d_y / delta_y.abs() {

    }else{
        if d_x%delta_x.abs() { }
    }

    println!("{} {} {d_x} {} {} {d_y}", prev_x, x_heap.peek().unwrap(), prev_y, y_heap.peek().unwrap());
}