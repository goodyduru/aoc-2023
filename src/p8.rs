use std::{fs, collections::HashMap};

struct Child {
    left: String,
    right: String
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p8.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return
    };
    first_puzzle(&input);
    second_puzzle(&input);
}

fn first_puzzle(input: &String) {
    let mut got_dir = false;
    let mut seen_comma = false;
    let mut directions = String::new();
    let mut first = String::with_capacity(3);
    let mut second = String::with_capacity(3);
    let mut item = String::with_capacity(3);
    let mut map = HashMap::new();

    for c in (input).chars() { 
        if c == '\n' {
            if item.len() == 3 {
                map.insert(item, Child{left: first, right: second});
                item = String::with_capacity(3);
                first = String::with_capacity(3);
                second = String::with_capacity(3);
            }
            if !got_dir {
                got_dir = true;
            }
            seen_comma = false;
        } else if c == ',' {
            seen_comma = true;
        } else if c.is_alphabetic() {
            if !got_dir {
                directions.push(c);
                continue;
            }
            if item.len() < 3 {
                item.push(c);
                continue;
            }
            if seen_comma {
                second.push(c);
            } else {
                first.push(c);
            }
        }
    }
    map.insert(item, Child{left: first, right: second});
    
    let mut found = false;
    let mut steps = 0;
    let mut current = "AAA";
    while !found {
        for c in directions.chars() {
            if current == "ZZZ" {
                found = true;
                break;
            }
            let child = map.get(current).unwrap();
            current = if c == 'L' {&child.left} else {&child.right};
            steps += 1;
        }
    }
    println!("Puzzle 1 soln: {steps}");
}

fn second_puzzle(input: &String) {
    let mut got_dir = false;
    let mut seen_comma = false;
    let mut directions = String::new();
    let mut first = String::with_capacity(3);
    let mut second = String::with_capacity(3);
    let mut item = String::with_capacity(3);
    let mut map = HashMap::new();
    let mut starts: Vec<String> = Vec::new();

    for c in (input).chars() { 
        if c == '\n' {
            if item.len() == 3 {
                if item.ends_with('A') {
                    starts.push(item.clone());
                }
                map.insert(item, Child{left: first, right: second});
                item = String::with_capacity(3);
                first = String::with_capacity(3);
                second = String::with_capacity(3);
            }
            if !got_dir {
                got_dir = true;
            }
            seen_comma = false;
        } else if c == ',' {
            seen_comma = true;
        } else if c.is_alphanumeric() {
            if !got_dir {
                directions.push(c);
                continue;
            }
            if item.len() < 3 {
                item.push(c);
                continue;
            }
            if seen_comma {
                second.push(c);
            } else {
                first.push(c);
            }
        }
    }
    if item.ends_with('A') {
        starts.push(item.clone());
    }
    map.insert(item, Child{left: first, right: second});

    let mut found: bool;
    let mut steps: Vec<u64> = Vec::new();
    let mut current: &str;
    for i in 0..starts.len() {
        current = &starts[i];
        let mut step = 0;
        found = false;
        while !found {
            for c in directions.chars() {
                if current.ends_with('Z') {
                    found = true;
                    break;
                }
                let child = map.get(current).unwrap();
                current = if c == 'L' {&child.left} else {&child.right};
                step += 1;
            }
        }
        steps.push(step);
    }
    let step = steps.into_iter().reduce(|a, b| lcm(a, b)).unwrap();
    println!("Puzzle 2 soln: {step}");
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut temp: u64;
    let mut a = a;
    let mut b = b;
    while b != 0 {
        temp = b;
        b = a % b;
        a = temp;
    }
    a
}
