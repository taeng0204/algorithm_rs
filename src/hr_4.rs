#[cfg(test)]
fn get_largest_palindrme(n: u32) -> u32 {
    let mut a: u32 = 999;
    let mut max: u32 = 1;
    while a >= 100 {
        let mut b: u32 = 999;
        let mut p: u32;

        while b >= a {
            p = a * b;
            if p <= max {
                break;
            }
            if is_palindrome(p) && p > max && p < n {
                max = p;
            }
            b -= 1;
        }
        a -= 1;
    }
    return max;
}

#[cfg(test)]
fn is_palindrome(num: u32) -> bool {
    let mut n = num;
    let mut rev: u32 = 0;
    while n > 0 {
        rev = 10 * rev + n % 10;
        n /= 10;
    }
    return num == rev;
}

#[test]
fn test() {
    assert_eq!(get_largest_palindrme(101110), 101101);
    assert_eq!(get_largest_palindrme(800000), 793397);
}
