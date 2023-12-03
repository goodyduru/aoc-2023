use std::fs;
enum Repr {
    Symbol,
    Digit(u32),
}

struct Part {
    pub value: Repr,
    pub start: i32,
    pub end: i32,
    pub added: bool,
    pub adjacent: i32,
    pub ratio: u32
}

impl Part {
    fn new(value: Repr, start: i32, end: i32) -> Self {
        Self {
            value,
            start,
            end,
            added: false,
            adjacent: 0,
            ratio: 1,
        }
    }
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p3.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return
    };
    first_puzzle(&input);
    second_puzzle(&input);
}

fn first_puzzle(input: &str) {
    let mut sum = 0;
    let mut start = -1;
    let mut count = 0;
    let mut digit: u32 = 0;
    let mut vec: Vec<Part> = Vec::new();
    let mut prev_vec: Vec<Part> = Vec::new();
    let mut temp_vec: Vec<Part>;
    let mut part: Part;

    for c in input.chars() {
        if !c.is_digit(10) && digit > 0 {
            part = Part::new(Repr::Digit(digit), start, count-1);
            vec.push(part);
            digit = 0;
            start = -1;
        }
        if c == '\n' {
            sum += sum_line(&mut prev_vec, &mut vec);
            prev_vec.clear();
            temp_vec = prev_vec;
            prev_vec = vec;
            vec = temp_vec;
            count = 0;
            start = -1;
            continue;
        } else if c.is_digit(10) {
            start = if start < 0 {count} else {start};
            digit = digit*10 + c.to_digit(10).unwrap();
        } else if c != '.' {
            part = Part::new(Repr::Symbol, count, 0);
            vec.push(part);
            start = -1;
        }
        count += 1;
    }
    if digit > 0 {
        part = Part::new(Repr::Digit(digit), start, count-1);
        vec.push(part);
    }
    sum += sum_line(&mut prev_vec, &mut vec);
    println!("First puzzle sum is {sum}");
}

fn sum_line(prev_vec: &mut Vec<Part>, current_vec: &mut Vec<Part>) -> u32 {
    let mut sum = 0;
    let mut prev_added = false;
    let mut next_added = false;
    let mut p: &Part;
    let mut c: &Part;
    let prev_len = prev_vec.len();
    let current_len = current_vec.len();

    for i in 0..current_len {
        { 
            c = &current_vec[i];
            if let Repr::Symbol = &c.value {
                if i > 0 {
                    p = &current_vec[i-1];
                    if let Repr::Digit(x) = &p.value {
                        if !p.added && p.end+1 == c.start {
                            sum += x;
                            prev_added = true;
                        }
                    }
                }

                if i < current_len-1 {
                    p = &current_vec[i+1];
                    if let Repr::Digit(x) = &p.value {
                        if !p.added && p.start-1 == c.start {
                            sum += x;
                            next_added = true;
                        }
                    }
                }
            }
        }
        if prev_added  {
            current_vec[i-1].added = true;
            prev_added = false;
        }
        if next_added {
            current_vec[i+1].added = true;
            next_added = false;
        }
    }

    for i in 0..prev_len {
        p = &prev_vec[i];
        match p.value {
            Repr::Symbol => {
                for j in 0..current_len {
                    c = &current_vec[j];
                    if c.start-1 > p.start {
                        break;
                    }
                    if let Repr::Digit(x) = &c.value {
                        if c.added {
                            continue;
                        }
                        if p.start == c.start || p.start+1 == c.start 
                        || p.start == c.end || p.start-1 == c.end || (p.start > c.start && p.start < c.end) {
                            sum += x;
                            current_vec[j].added = true
                        }
                    }
                }
            },
            Repr::Digit(x) => {
                if p.added {
                    continue;
                }
                for j in 0..current_len {
                    c = &current_vec[j];
                    if p.start-1 > c.start {
                        continue;
                    }
                    if let Repr::Symbol = &c.value {
                        if c.start == p.start || c.start+1 == p.start 
                        || c.start == p.end || c.start-1 == p.end || (c.start > p.start && c.start < p.end) {
                            sum += x;
                        }
                    }
                }
            }
        }
    }
    sum
}



fn second_puzzle(input: &str) {
    let mut sum = 0;
    let mut start = -1;
    let mut count = 0;
    let mut digit: u32 = 0;
    let mut vec: Vec<Part> = Vec::new();
    let mut prev_vec: Vec<Part> = Vec::new();
    let mut temp_vec: Vec<Part>;
    let mut part: Part;

    for c in input.chars() {
        if !c.is_digit(10) && digit > 0 {
            part = Part::new(Repr::Digit(digit), start, count-1);
            vec.push(part);
            digit = 0;
            start = -1;
        }
        if c == '\n' {
            sum += sum_gears(&mut prev_vec, &mut vec);
            prev_vec.clear();
            temp_vec = prev_vec;
            prev_vec = vec;
            vec = temp_vec;
            count = 0;
            start = -1;
            continue;
        } else if c.is_digit(10) {
            start = if start < 0 {count} else {start};
            digit = digit*10 + c.to_digit(10).unwrap();
        } else if c == '*' {
            part = Part::new(Repr::Symbol, count, 0);
            vec.push(part);
            start = -1;
        }
        count += 1;
    }
    if digit > 0 {
        part = Part::new(Repr::Digit(digit), start, count-1);
        vec.push(part);
    }
    sum += sum_gears(&mut prev_vec, &mut vec);
    println!("Second puzzle sum is {sum}");
}

fn sum_gears(prev_vec: &mut Vec<Part>, current_vec: &mut Vec<Part>) -> u32 {
    let mut sum = 0;
    let mut prev_val = 0;
    let mut next_val = 0;
    let mut p: &Part;
    let mut c: &Part;
    let prev_len = prev_vec.len();
    let current_len = current_vec.len();

    for i in 0..current_len {
        { 
            c = &current_vec[i];
            if let Repr::Symbol = &c.value {
                if i > 0 {
                    p = &current_vec[i-1];
                    if let Repr::Digit(x) = &p.value {
                        if p.end+1 == c.start {
                            prev_val = *x;
                        }
                    }
                }

                if i < current_len-1 {
                    p = &current_vec[i+1];
                    if let Repr::Digit(x) = &p.value {
                        if p.start-1 == c.start {
                            next_val = *x;
                        }
                    }
                }
            }
        }
        if prev_val > 0  {
            current_vec[i].adjacent += 1;
            current_vec[i].ratio *= prev_val;
            prev_val = 0;
        }
        if next_val > 0 {
            current_vec[i].adjacent += 1;
            current_vec[i].ratio *= next_val;
            next_val = 0;
        }
    }

    let mut p: &mut Part;
    let mut c: &mut Part;
    for i in 0..prev_len {
        p = &mut prev_vec[i];
        match p.value {
            Repr::Symbol => {
                for j in 0..current_len {
                    c = &mut current_vec[j];
                    if c.start-1 > p.start {
                        break;
                    }
                    if let Repr::Digit(x) = &c.value {
                        if p.start == c.start || p.start+1 == c.start 
                        || p.start == c.end || p.start-1 == c.end || (p.start > c.start && p.start < c.end) {
                            p.adjacent += 1;
                            if p.adjacent <= 2 {
                                p.ratio *= x;
                            }
                        }
                    }
                }
                if p.adjacent == 2 {
                    sum += p.ratio;
                }
            },
            Repr::Digit(x) => {
                for j in 0..current_len {
                    c = &mut current_vec[j];
                    if p.start-1 > c.start {
                        continue;
                    }
                    if let Repr::Symbol = &c.value {
                        if c.start == p.start || c.start+1 == p.start 
                        || c.start == p.end || c.start-1 == p.end || (c.start > p.start && c.start < p.end) {
                            c.adjacent += 1;
                            if c.adjacent <= 2 {
                                c.ratio *= x;
                            }
                        }
                    }
                }
            }
        }
    }
    sum
}