use std::collections::VecDeque;
use std::io::{self, Read};
const MAX_NUM: usize = 10000;

struct Dslr {
    visited: [bool; MAX_NUM],
    prev: [i16; MAX_NUM],
    how: [u8; MAX_NUM],
    queue: VecDeque<i16>,
}

impl Dslr {
    fn new() -> Self {
        Self {
            visited: [false; MAX_NUM],
            prev: [-1; MAX_NUM],
            how: [0; MAX_NUM],
            queue: VecDeque::with_capacity(MAX_NUM),
        }
    }

    #[inline]
    fn reset(&mut self) {
        self.visited.fill(false);
        self.prev.fill(-1);
        self.how.fill(0);
        self.queue.clear();
    }

    #[inline]
    fn push(&mut self, next: i16, cur: i16, op: u8) {
        let u = next as usize;
        if !self.visited[u] {
            self.visited[u] = true;
            self.prev[u] = cur;
            self.how[u] = op;
            self.queue.push_back(next);
        }
    }

    fn run(&mut self, start: usize, target: usize) -> String {
        if start == target {
            return String::new();
        }

        self.reset();
        self.visited[start] = true;
        self.queue.push_back(start as i16);

        while let Some(cur) = self.queue.pop_front() {
            if cur as usize == target {
                break;
            }

            let d = (cur * 2) % 10000;
            let s = if cur == 0 { 9999 } else { cur - 1 };
            let l = ((cur % 1000) * 10) + (cur / 1000);
            let r = ((cur % 10) * 1000) + (cur / 10);

            self.push(d, cur, b'D');
            self.push(s, cur, b'S');
            self.push(l, cur, b'L');
            self.push(r, cur, b'R');
        }

        let mut path = Vec::new();
        let mut cur = target as i16;
        while cur != start as i16 {
            path.push(self.how[cur as usize] as char);
            cur = self.prev[cur as usize];
        }
        path.reverse();
        path.into_iter().collect()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut dslr = Dslr::new();
    let mut output = String::new();

    for _ in 0..n {
        let start = iter.next().unwrap().parse().unwrap();
        let end = iter.next().unwrap().parse().unwrap();
        let result = dslr.run(start, end);
        output.push_str(&result);
        output.push('\n');
    }
    println!("{}", output);
}
