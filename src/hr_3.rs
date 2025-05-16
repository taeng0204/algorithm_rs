#[cfg(test)]
fn get_largest_prime_factor(n: u64) -> u64 {
    let mut p = 2;
    let mut r = n;
    while r / p > 1 && p <= (r as f64).sqrt() as u64 {
        if r % p == 0 {
            r /= p;
        } else {
            p += 1;
        }
    }
    r
}

#[test]
fn test() {
    assert_eq!(get_largest_prime_factor(10), 5);
    assert_eq!(get_largest_prime_factor(17), 17);
    assert_eq!(get_largest_prime_factor(38), 19);
}
