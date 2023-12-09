use std::{fs, time::Instant};

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p9.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return,
    };
    let now = Instant::now();
    puzzle(&input, true);
    puzzle(&input, false);
    let elapsed = Instant::elapsed(&now);
    println!("{:?}", elapsed);
}

fn puzzle(input: &String, is_next: bool) {
    let mut digit: i64 = 0;
    let mut neg: i64 = 1;
    let mut sequence: Vec<i64> = Vec::new();
    let mut sum = 0;
    for c in input.chars() {
        if c == '\n' {
            digit *= neg;
            sequence.push(digit);
            sum += interpol(&mut sequence, is_next);
            sequence.clear();
            neg = 1;
            digit = 0;
        } else if c == '-' {
            neg = -1;
        } else if c == ' ' {
            digit *= neg;
            sequence.push(digit);
            neg = 1;
            digit = 0;
        } else {
            digit = digit * 10 + c.to_digit(10).unwrap() as i64;
        }
    }
    digit *= neg;
    sequence.push(digit);
    sum += interpol(&mut sequence, is_next);
    println!("Puzzle soln: {sum}");
}

fn interpol(sequence: &mut Vec<i64>, is_next: bool) -> i64 {
    let mut n = 0;
    let x = sequence.len() as i64;
    let mut is_same = false;
    let mut coef: Vec<i64> = Vec::new();
    coef.push(sequence[0]);
    while !is_same {
        is_same = true;
        for i in 1..sequence.len() - n {
            if !is_same {
                break;
            }
            if sequence[i] != sequence[i-1] {
                is_same = false;
            }
        }
        if is_same {
            break;
        }

        for i in 1..sequence.len() - n {
            sequence[i-1] = sequence[i] - sequence[i-1];
        }
        coef.push(sequence[0]);
        n += 1;
    }

    if is_next {
        p(coef, x)
    }
    else {
        prev(coef)
    }

}

fn p(coef: Vec<i64>, x: i64) -> i64 {
    let mut ans: i64 = 0;
    for i in 0..coef.len() {
        ans += coef[i]*g(i as i64, x);
    }
    ans
}

fn g(r: i64, x: i64) -> i64 {
    let mut ans: i64 = 1;
    let mut fact: i64 = 1;
    for i in 1..=r {

        fact *= i;
        ans *= x-i+1;
    }

    ans/fact
}

fn prev(coef: Vec<i64>) -> i64 {
    let mut diff: i64 = 0;
    for i in {0..coef.len()}.rev() {
        diff = coef[i] - diff;
    }
    diff
}