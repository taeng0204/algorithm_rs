#[cfg(test)]
fn get_sum(bound: u64) -> u64 {
    return sum_of_multiples(3, bound - 1) + sum_of_multiples(5, bound - 1)
        - sum_of_multiples(15, bound - 1);
}

#[cfg(test)]
fn sum_of_multiples(n: u64, k: u64) -> u64 {
    let count = k / n;
    n * (count * (count + 1) / 2)
}

#[test]
fn test() {
    assert_eq!(get_sum(10), 23);
    assert_eq!(get_sum(1000), 233168);
    assert_eq!(get_sum(1000000000), 233333333166666668);
}
