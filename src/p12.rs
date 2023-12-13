use std::{collections::HashMap, fs, time::Instant};

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p12.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return,
    };
    let now = Instant::now();
    first_puzzle(&input);
    second_puzzle(&input);
    let elapsed = Instant::elapsed(&now);
    println!("{:?}", elapsed);
}

fn first_puzzle(input: &String) {
    let mut sum = 0;
    let mut digit = 0;
    let mut record = String::new();
    let mut groups: Vec<usize> = Vec::new();
    let mut checked: HashMap<(usize, usize, usize), u64> = HashMap::new();

    for c in input.chars() {
        if c == '\n' {
            if digit > 0 {
                groups.push(digit);
            }
            let s = calculate(&record, &groups, &mut checked, 0, 0, 0);
            sum += s;
            record.clear();
            groups.clear();
            checked.clear();
            digit = 0;
            continue;
        } else if c == '#' || c == '?' || c == '.' {
            record.push(c);
        } else if c.is_digit(10) {
            digit = digit * 10 + c.to_digit(10).unwrap() as usize;
        } else if c == ',' {
            groups.push(digit);
            digit = 0;
        }
    }
    if digit > 0 {
        groups.push(digit);
    }
    let s = calculate(&record, &groups, &mut checked, 0, 0, 0);
    sum += s;
    println!("First puzzle: {sum}");
}

fn second_puzzle(input: &String) {
    let mut sum: u64 = 0;
    let mut digit = 0;
    let mut record = String::new();
    let mut groups: Vec<usize> = Vec::new();
    let mut checked: HashMap<(usize, usize, usize), u64> = HashMap::new();

    for c in input.chars() {
        if c == '\n' {
            if digit > 0 {
                groups.push(digit);
            }
            let new_record = record.clone();
            let new_groups = groups.clone();
            for _ in 0..4 {
                record.push('?');
                record.extend(new_record.chars());
                groups.extend(new_groups.iter());
            }
            let s = calculate(&record, &groups, &mut checked, 0, 0, 0);
            sum += s;
            record.clear();
            groups.clear();
            checked.clear();
            digit = 0;
            continue;
        } else if c == '#' || c == '?' || c == '.' {
            record.push(c);
        } else if c.is_digit(10) {
            digit = digit * 10 + c.to_digit(10).unwrap() as usize;
        } else if c == ',' {
            groups.push(digit);
            digit = 0;
        }
    }
    if digit > 0 {
        groups.push(digit);
    }
    let new_record = record.clone();
    let new_groups = groups.clone();
    for _ in 0..4 {
        record.push('?');
        record.extend(new_record.chars());
        groups.extend(new_groups.iter());
    }
    let s = calculate(&record, &groups, &mut checked, 0, 0, 0);
    sum += s;
    println!("First puzzle: {sum}");
}

fn calculate(
    record: &String,
    groups: &Vec<usize>,
    checked: &mut HashMap<(usize, usize, usize), u64>,
    chpos: usize,
    numpos: usize,
    hashes: usize,
) -> u64 {
    let key = (chpos, numpos, hashes);
    if checked.contains_key(&key) {
        return checked[&key];
    }
    if chpos == record.len() {
        if groups.len() == numpos && hashes == 0 {
            return 1;
        }
        if groups.len() - 1 == numpos && hashes == groups[numpos] {
            return 1;
        }
        return 0;
    }

    let mut answer = 0;
    let options = ".#";
    for c in options.chars() {
        let d = record.chars().nth(chpos as usize).unwrap();
        if d == c || d == '?' {
            if c == '.' && hashes == 0 {
                answer += calculate(record, groups, checked, chpos + 1, numpos, hashes);
            } else if c == '.' && hashes > 0 && numpos < groups.len() && groups[numpos] == hashes {
                answer += calculate(record, groups, checked, chpos + 1, numpos + 1, 0)
            } else if c == '#' {
                answer += calculate(record, groups, checked, chpos + 1, numpos, hashes + 1)
            }
        }
    }
    checked.insert(key, answer);
    return answer;
}
