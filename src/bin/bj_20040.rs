use std::cmp::Ordering;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut parent: Vec<usize> = (0..n).collect();
    let mut result = 0;

    for i in 1..=m {
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();

        let top_u = find(&mut parent, u);
        let top_v = find(&mut parent, v);

        match top_u.cmp(&top_v) {
            Ordering::Equal => {
                result = i;
                break;
            }
            Ordering::Less => {
                parent[top_v] = top_u;
            }
            Ordering::Greater => {
                parent[top_u] = top_v;
            }
        }
    }
    println!("{}", result);
}

fn find(parent: &mut Vec<usize>, i: usize) -> usize {
    if parent[i] == i {
        i
    } else {
        parent[i] = find(parent, parent[i]);
        parent[i]
    }
}
