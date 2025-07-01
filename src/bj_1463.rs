#[cfg(test)]
fn count_operation_to_one(n: usize) -> usize {
    let mut count = vec![0; n + 1];
    for i in 2..=n {
        count[i] = count[i - 1] + 1;
        if i % 2 == 0 {
            count[i] = count[i].min(count[i / 2] + 1);
        }
        if i % 3 == 0 {
            count[i] = count[i].min(count[i / 3] + 1);
        }
    }
    count[n]
}
// 10 1 3 9 10
#[test]
fn test() {
    assert_eq!(count_operation_to_one(2), 1);
    assert_eq!(count_operation_to_one(10), 3);
}

// use std::io::{self, Read};

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_to_string(&mut input).unwrap();

//     let n: usize = input.trim().parse().unwrap();
//     println!("{}", count_operation_to_one(n));
// }
