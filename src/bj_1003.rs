#[cfg(test)]
fn count_fibonacci(n: usize) -> (usize, usize) {
    if n == 0 {
        return (1, 0);
    }

    let mut prev = (1, 0);
    let mut current = (0, 1);

    for _ in 2..=n {
        let next = (prev.0 + current.0, prev.1 + current.1);
        prev = current;
        current = next;
    }
    current
}

#[test]
fn test() {
    assert_eq!(count_fibonacci(0), (1, 0));
    assert_eq!(count_fibonacci(1), (0, 1));
    assert_eq!(count_fibonacci(3), (1, 2));
}

// use std::io::{self, Read};

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_to_string(&mut input).unwrap();

//     let mut lines = input.lines();
//     let t: usize = lines.next().unwrap().parse().unwrap();

//     for _ in 0..t {
//         let n: usize = lines.next().unwrap().parse().unwrap();
//         let (count_0, count_1) = count_fibonacci(n);
//         println!("{} {}", count_0, count_1);
//     }
// }
//
