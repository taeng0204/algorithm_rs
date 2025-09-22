use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let caps: [usize; 3] = [
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    ];

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut result = vec![false; caps[2] + 1];

    let start = [0, 0, caps[2]];
    queue.push_back(start);
    visited.insert(start);

    while let Some(cups) = queue.pop_front() {
        if cups[0] == 0 {
            result[cups[2]] = true;
        }
        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    continue;
                }
                let mut next_cups = cups;

                let pour = next_cups[i].min(caps[j] - next_cups[j]);
                next_cups[i] -= pour;
                next_cups[j] += pour;

                if visited.insert(next_cups) {
                    queue.push_back(next_cups);
                }
            }
        }
    }

    let result: Vec<String> = result
        .iter()
        .enumerate()
        .filter(|&(_, &yes)| yes)
        .map(|(water, _)| water.to_string())
        .collect();

    println!("{}", result.join(" "));
}
