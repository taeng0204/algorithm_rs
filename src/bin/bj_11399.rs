use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let _: usize = iter.next().unwrap().parse().unwrap();
    let mut nums: Vec<u32> = iter.map(|x| x.parse().unwrap()).collect();
    nums.sort_unstable();
    let mut sum = 0;
    let mut total = 0;
    for num in nums {
        sum += num;
        total += sum;
    }
    println!("{}", total);
}
