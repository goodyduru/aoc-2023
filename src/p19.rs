use std::{collections::HashMap, fs, time::Instant};

#[derive(Debug)]
struct Rule {
    part: char,
    is_greater: bool,
    to: u32,
    result: String,
}

impl Rule {
    fn new() -> Self {
        Self {
            part: ' ',
            is_greater: true,
            to: 0,
            result: String::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Range {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p19.txt");
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
    let mut total = 0;
    let mut is_workflow = true;
    let mut map: HashMap<String, Vec<Rule>> = HashMap::new();

    input.lines().for_each(|line| {
        if line == "" {
            is_workflow = false;
            return;
        }
        if is_workflow {
            parse_workflow(line, &mut map);
        } else {
            total += process_rating(line, &map);
        }
    });
    println!("Total: {total}");
}

fn parse_workflow(line: &str, map: &mut HashMap<String, Vec<Rule>>) {
    let mut seen_bracket = false;
    let mut is_greater = false;
    let mut seen_colon = false;
    let mut seen_sign = false;
    let mut digit = 0;
    let mut part = ' ';
    let mut result = String::new();
    let mut key = String::new();
    let mut rule = Rule::new();
    let mut rules = Vec::new();
    for c in line.chars() {
        if c == '{' {
            seen_bracket = true;
        } else if c == '}' {
            break;
        } else if c == ',' {
            rule.part = part;
            rule.result = result;
            rule.to = digit;
            rule.is_greater = is_greater;
            rules.push(rule);
            part = ' ';
            is_greater = false;
            digit = 0;
            result = String::new();
            seen_colon = false;
            seen_sign = false;
            rule = Rule::new();
        } else if !seen_bracket {
            key.push(c);
        } else if c == '>' || c == '<' {
            if c == '>' {
                is_greater = true;
            }
            part = result.chars().nth(0).unwrap();
            result.clear();
            seen_sign = true;
        } else if !seen_sign {
            result.push(c);
        } else if c.is_digit(10) {
            digit = digit * 10 + c.to_digit(10).unwrap();
        } else if c == ':' {
            seen_colon = true;
        } else if seen_colon {
            result.push(c);
        }
    }

    rule.part = part;
    rule.result = result;
    rule.to = digit;
    rule.is_greater = is_greater;
    rules.push(rule);
    map.insert(key, rules);
}

fn process_rating(line: &str, map: &HashMap<String, Vec<Rule>>) -> u64 {
    let mut index = 0;
    let mut parts = [0; 4]; //x m a s
    for c in line.chars() {
        if c == ',' {
            index += 1;
        } else if c.is_digit(10) {
            parts[index] = parts[index] * 10 + c.to_digit(10).unwrap();
        }
    }

    let mut key = "in";
    while map.contains_key(key) {
        for rule in &map[key] {
            if rule.part == 'x' {
                if rule.is_greater {
                    if parts[0] > rule.to {
                        key = &rule.result;
                        break;
                    }
                } else {
                    if parts[0] < rule.to {
                        key = &rule.result;
                        break;
                    }
                }
            } else if rule.part == 'm' {
                if rule.is_greater {
                    if parts[1] > rule.to {
                        key = &rule.result;
                        break;
                    }
                } else {
                    if parts[1] < rule.to {
                        key = &rule.result;
                        break;
                    }
                }
            } else if rule.part == 'a' {
                if rule.is_greater {
                    if parts[2] > rule.to {
                        key = &rule.result;
                        break;
                    }
                } else {
                    if parts[2] < rule.to {
                        key = &rule.result;
                        break;
                    }
                }
            } else if rule.part == 's' {
                if rule.is_greater {
                    if parts[3] > rule.to {
                        key = &rule.result;
                        break;
                    }
                } else {
                    if parts[3] < rule.to {
                        key = &rule.result;
                        break;
                    }
                }
            } else {
                key = &rule.result;
            }
        }
        if key == "A" {
            return (parts[0] + parts[1] + parts[2] + parts[3]) as u64;
        } else if key == "R" {
            return 0;
        }
    }
    0
}

fn second_puzzle(input: &String) {
    let mut is_workflow = true;
    let mut map: HashMap<String, Vec<Rule>> = HashMap::new();

    input.lines().for_each(|line| {
        if line == "" {
            is_workflow = false;
            return;
        }
        if is_workflow {
            parse_workflow(line, &mut map);
        }
    });

    let total = combinator(&map);
    println!("Total: {total}");
}

fn combinator(map: &HashMap<String, Vec<Rule>>) -> u64 {
    let mut ranges: HashMap<&str, Range> = HashMap::new();
    ranges.insert(
        "in",
        Range {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    );
    let mut keys = vec!["in"];
    let mut accepted = vec![];

    while keys.len() > 0 {
        let key = keys.pop().unwrap();
        let rules = &map[key];
        let mut range = ranges[key].clone();
        for rule in rules {
            let mut new_range = range.clone();
            let to = rule.to as u64;
            if rule.part == 'x' {
                if rule.is_greater {
                    new_range.x.0 = to + 1;
                    range.x.1 = to;
                } else {
                    new_range.x.1 = to - 1;
                    range.x.0 = to;
                }
            } else if rule.part == 'm' {
                if rule.is_greater {
                    new_range.m.0 = to + 1;
                    range.m.1 = to;
                } else {
                    new_range.m.1 = to - 1;
                    range.m.0 = to;
                }
            } else if rule.part == 'a' {
                if rule.is_greater {
                    new_range.a.0 = to + 1;
                    range.a.1 = to;
                } else {
                    new_range.a.1 = to - 1;
                    range.a.0 = to;
                }
            } else if rule.part == 's' {
                if rule.is_greater {
                    new_range.s.0 = to + 1;
                    range.s.1 = to;
                } else {
                    new_range.s.1 = to - 1;
                    range.s.0 = to;
                }
            }

            if rule.result == "A" {
                accepted.push(new_range);
            } else if rule.result != "R" {
                ranges.insert(&(rule.result), new_range);
                keys.push(&rule.result);
            }
        }
    }

    let mut total = 0;
    for range in accepted {
        total += (1 + range.x.1 - range.x.0)
            * (1 + range.m.1 - range.m.0)
            * (1 + range.a.1 - range.a.0)
            * (1 + range.s.1 - range.s.0);
    }
    total
}
