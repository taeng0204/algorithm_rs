use std::{
    io::{self, Read},
    usize,
};

struct Heap {
    data: Vec<usize>,
}

impl Heap {
    fn new() -> Self {
        Heap { data: Vec::new() }
    }

    fn push(&mut self, value: usize) {
        self.data.push(value);
        self.heapify_up();
    }

    fn pop(&mut self) -> usize {
        if self.data.is_empty() {
            return 0;
        }
        let min = self.data[0];
        let last = self.data.pop().unwrap();
        if !self.data.is_empty() {
            self.data[0] = last;
            self.heapify_down();
        }
        min
    }

    fn heapify_up(&mut self) {
        let mut i = self.data.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[parent] > self.data[i] {
                self.data.swap(parent, i);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self) {
        let mut i = 0;
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;

            if left < self.data.len() && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < self.data.len() && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            if smallest != i {
                self.data.swap(i, smallest);
                i = smallest;
            } else {
                break;
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines().map(|s| s.trim().parse::<usize>().unwrap());

    let n = lines.next().unwrap();
    let mut heap = Heap::new();
    let mut output = String::new();

    for num in lines.take(n) {
        if num == 0 {
            output.push_str(&format!("{}\n", heap.pop()));
        } else {
            heap.push(num);
        }
    }
    print!("{}", output);
}
