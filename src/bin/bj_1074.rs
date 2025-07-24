use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse::<i32>().unwrap();
    let r = iter.next().unwrap().parse().unwrap();
    let c = iter.next().unwrap().parse().unwrap();

    let result = get_result(n, r, c);
    println!("{}", result);
}

fn get_result(n: i32, r: i32, c: i32) -> i32 {
    let half = 1 << (n - 1);
    find_num_in_z(half, r, c)
}

fn find_num_in_z(half: i32, r: i32, c: i32) -> i32 {
    let mut result = 0;
    let mut nr = r;
    let mut nc = c;

    if r >= half {
        result += half.pow(2) * 2;
        nr = r % half;
        if c >= half {
            result += half.pow(2);
            nc = c % half;
        }
    } else {
        if c >= half {
            result += half.pow(2);
            nc = c % half;
        }
    }

    if half == 1 {
        return result;
    } else {
        result += find_num_in_z(half / 2, nr, nc);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_cases() {
        assert_eq!(get_result(1, 0, 0), 0);
        assert_eq!(get_result(2, 3, 1), 11);
        assert_eq!(get_result(3, 7, 7), 63);
        assert_eq!(get_result(4, 7, 7), 63);
    }

    #[test]
    fn test_large_cases() {
        assert_eq!(get_result(10, 511, 511), 262143);
        assert_eq!(get_result(10, 512, 512), 786432);
    }
}
