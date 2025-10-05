use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let func = iter.next().unwrap().to_string();
        let _ = iter.next().unwrap();
        let arr = iter.next().unwrap();

        let mut arr: VecDeque<&str> = arr[1..arr.len() - 1]
            .split(',')
            .filter(|c| !c.trim().is_empty())
            .collect();
        let mut is_reversed = false;
        let mut is_broken = false;
        for c in func.chars() {
            match c {
                'R' => is_reversed = !is_reversed,
                'D' => {
                    if arr.is_empty() {
                        println!("error");
                        is_broken = true;
                        break;
                    }
                    if is_reversed {
                        arr.pop_back();
                    } else {
                        arr.pop_front();
                    }
                }
                _ => unreachable!(),
            }
        }
        if is_broken {
            continue;
        }
        let slice = arr.make_contiguous();
        if is_reversed {
            slice.reverse();
        }
        println!("[{}]", slice.join(","));
    }
}
