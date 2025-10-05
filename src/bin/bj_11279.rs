use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut heap = BinaryHeap::new();
    let mut output = String::new();
    for _ in 0..n {
        let num: u32 = iter.next().unwrap().parse().unwrap();
        if num == 0 {
            if let Some(max) = heap.pop() {
                output.push_str(&format!("{}\n", max));
            } else {
                output.push_str("0\n");
            }
        } else {
            heap.push(num);
        }
    }
    print!("{}", output);
}
