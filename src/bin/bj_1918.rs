use std::io::{self, Read};

fn weight(op: char) -> usize {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();
    let mut stack: Vec<char> = Vec::new();

    for c in input.chars() {
        match c {
            'A'..='Z' => {
                out.push(c);
            }
            '(' => {
                stack.push(c);
            }
            ')' => {
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    out.push(top);
                }
            }
            '+' | '-' | '*' | '/' => {
                while let Some(&top) = stack.last() {
                    if top == '(' {
                        break;
                    }
                    if weight(top) >= weight(c) {
                        out.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(c);
            }
            _ => {}
        }
    }

    while let Some(op) = stack.pop() {
        if op != '(' {
            out.push(op);
        }
    }

    println!("{}", out);
}
