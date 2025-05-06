#[cfg(test)]
fn get_fibo_even_sum(bound: u64) -> u64 {
    let mut sum = 0;
    let mut x = 1;
    let mut y = 1;
    let mut z = x + y;
    while z <= bound {
        if z % 2 == 0 {
            sum += z;
        }
        x = y;
        y = z;
        z = x + y;
    }
    sum
}

#[test]
fn test() {
    assert_eq!(get_fibo_even_sum(10), 10);
    assert_eq!(get_fibo_even_sum(4_000_000), 4_613_732);
    assert_eq!(
        get_fibo_even_sum(40_000_000_000_000_000),
        49_597_426_547_377_748
    );
}
