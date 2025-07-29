use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut comb: Vec<usize> = (1..=m).collect();

    loop {
        println!(
            "{}",
            comb.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

        let mut i = m;
        let mut found = false;

        while i > 0 {
            i -= 1;
            // 각 자리(i)에서의 최댓값 = n에서.. 뒤에서 m 만큼 -, i번째 만큼 +, 기본 1 +
            if comb[i] < n - m + i + 1 {
                comb[i] += 1;
                // 현재 증가하는 자리가 중간이라면.. 그 뒤 값을 땡겨옴
                for j in i + 1..m {
                    comb[j] = comb[j - 1] + 1;
                }
                found = true;
                break;
            }
        }

        if !found {
            break;
        }
    }
}
