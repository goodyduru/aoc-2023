use std::{
    collections::{HashMap, VecDeque},
    fs,
    time::Instant,
};

#[derive(Debug, PartialEq)]
enum Type {
    BROADCASTER,
    CONJUCTION,
    FLIPFLOP,
}

#[derive(Debug)]
struct Module {
    category: Type,
    state: bool,
    output: Vec<usize>,
    inputs: HashMap<usize, bool>,
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p20.txt");
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
    let (mut map, broadcast, _) = parse_input(input);
    let mut high_pulse: usize = 0;
    let mut low_pulse: usize = 0;
    // Pulse -> Low: False, High: True
    for _ in 0..1000 {
        let mut stack = VecDeque::new();
        stack.push_back((0, broadcast, false));
        while let Some((from, index, pulse)) = stack.pop_front() {
            if pulse {
                high_pulse += 1;
            } else {
                low_pulse += 1;
            }
            if !map.contains_key(&index) {
                continue;
            }
            match map[&index].category {
                Type::BROADCASTER => {
                    handle_broadcast(&map[&index], index, pulse, &mut stack);
                }
                Type::CONJUCTION => {
                    handle_conjuction(map.get_mut(&index).unwrap(), index, from, pulse, &mut stack)
                }
                Type::FLIPFLOP => {
                    handle_flipflop(map.get_mut(&index).unwrap(), index, pulse, &mut stack)
                }
            }
        }
    }
    let total = high_pulse * low_pulse;
    println!("Puzzle 1 soln: {total}");
}

fn parse_input(input: &String) -> (HashMap<usize, Module>, usize, usize) {
    let mut map: HashMap<usize, Module> = HashMap::new();
    let mut reverse_index: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut all: Vec<String> = vec!["".to_string()];
    let mut broadcast = 0;
    let mut rx_index = 0;
    input.lines().for_each(|line| {
        let mut iter = line.split(" -> ");
        let raw_key = iter.next().unwrap();
        let values: Vec<&str> = iter.next().unwrap().split(", ").collect();
        let mut offset = 0;
        let mut prefix = ' ';
        if raw_key.contains('%') {
            prefix = '%';
            offset = 1;
        } else if raw_key.contains('&') {
            prefix = '&';
            offset = 1;
        }
        let key = raw_key[offset..].to_string();
        let found = all.iter().position(|x| x == &key);
        let key_index;
        match found {
            Some(i) => key_index = i,
            None => {
                key_index = all.len();
                all.push(key);
            }
        }
        let mut module = Module {
            category: Type::FLIPFLOP,
            state: false,
            output: Vec::new(),
            inputs: HashMap::new(),
        };
        if prefix == '&' {
            module.category = Type::CONJUCTION;
        } else if prefix == ' ' {
            module.category = Type::BROADCASTER;
            broadcast = key_index;
        }

        for v in values {
            let found = all.iter().position(|x| x == v);
            let index: usize;
            match found {
                Some(i) => {
                    module.output.push(i);
                    index = i;
                }
                None => {
                    index = all.len();
                    if v == "rx" {
                        rx_index = key_index;
                    }
                    all.push(v.to_string());
                    module.output.push(index);
                }
            }
            if reverse_index.contains_key(&index) {
                reverse_index.get_mut(&index).unwrap().push(key_index);
            } else {
                reverse_index.insert(index, vec![key_index]);
            }
        }
        map.insert(key_index, module);
    });

    for (key, val) in map.iter_mut() {
        if val.category == Type::CONJUCTION {
            let v = reverse_index.get(&key).unwrap();
            for i in v {
                (*val).inputs.insert(*i, false);
            }
        }
    }
    (map, broadcast, rx_index)
}

fn handle_broadcast(
    module: &Module,
    index: usize,
    pulse: bool,
    stack: &mut VecDeque<(usize, usize, bool)>,
) {
    for i in &module.output {
        stack.push_back((index, *i, pulse));
    }
}

fn handle_flipflop(
    module: &mut Module,
    index: usize,
    pulse: bool,
    stack: &mut VecDeque<(usize, usize, bool)>,
) {
    if pulse {
        return;
    }
    module.state = !module.state;
    let output = module.state;
    for i in &module.output {
        stack.push_back((index, *i, output));
    }
}

fn handle_conjuction(
    module: &mut Module,
    index: usize,
    from: usize,
    pulse: bool,
    stack: &mut VecDeque<(usize, usize, bool)>,
) {
    module.inputs.insert(from, pulse);
    let mut output = false;
    for (_, state) in module.inputs.iter() {
        if !state {
            output = true;
            break;
        }
    }
    for i in &module.output {
        stack.push_back((index, *i, output));
    }
}

fn second_puzzle(input: &String) {
    let (mut map, broadcast, parent) = parse_input(input);
    let mut temp: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut i = 0;

    // Pulse -> Low: False, High: True
    while i < 10000 {
        i += 1;
        let mut stack = VecDeque::new();
        stack.push_back((0, broadcast, false));
        while let Some((from, index, pulse)) = stack.pop_front() {
            if !map.contains_key(&index) {
                continue;
            }
            if map[&parent].inputs.contains_key(&from) && pulse {
                if temp.contains_key(&from) {
                    temp.get_mut(&from).unwrap().push(i);
                } else {
                    temp.insert(from, vec![i]);
                }
            }
            match map[&index].category {
                Type::BROADCASTER => {
                    handle_broadcast(&map[&index], index, pulse, &mut stack);
                }
                Type::CONJUCTION => {
                    handle_conjuction(map.get_mut(&index).unwrap(), index, from, pulse, &mut stack)
                }
                Type::FLIPFLOP => {
                    handle_flipflop(map.get_mut(&index).unwrap(), index, pulse, &mut stack)
                }
            }
        }
    }
    let values: Vec<u64> = temp.iter().map(|(_, v)| v[0] as u64).collect();
    let lcm = values.into_iter().reduce(|a, b| lcm(a, b)).unwrap();
    println!("Puzzle 2 soln: {lcm}");
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
