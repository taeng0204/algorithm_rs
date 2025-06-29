#[cfg(test)]
fn count_reminder_sum(_: usize, m: usize, numbers: &[usize]) -> usize {
    let mut remain_count: Vec<usize> = vec![0; m];
    remain_count[0] = 1;

    let mut sum = 0;
    for num in numbers {
        sum = (sum + num) % m;
        remain_count[sum] += 1;
    }
    remain_count
        .iter()
        .map(|&count| {
            if count > 1 {
                count * (count - 1) / 2
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn test() {
    assert_eq!(count_reminder_sum(5, 3, &[1, 2, 3, 1, 2]), 7);
}

// use std::io::{self, Read};

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_to_string(&mut input).unwrap();

//     let mut iter = input
//         .split_whitespace()
//         .map(|s| s.parse::<usize>().unwrap());

//     let n = iter.next().unwrap();
//     let m = iter.next().unwrap();

//     let numbers: Vec<usize> = iter.collect();

//     println!("{}", count_reminder_sum(n, m, &numbers));
// }
