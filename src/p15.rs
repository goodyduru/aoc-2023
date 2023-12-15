use std::{cmp::Ordering, fs, time::Instant};

struct Lens {
    label: String,
    focal_length: u8,
    index: usize,
}

impl Lens {
    fn get_index(&self) -> usize {
        self.index
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(&other.label)
    }
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p15.txt");
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
    let mut total: u64 = 0;
    let mut current: u64 = 0;

    for c in input.bytes() {
        if c == b',' {
            total += current;
            current = 0;
        } else {
            current += c as u64;
            current *= 17;
            current %= 256;
        }
    }
    total += current;
    println!("Part 1 soln: {total}");
}

fn second_puzzle(input: &String) {
    let mut focal_length = 0;
    let mut current: usize = 0;
    let mut index: usize = 0;
    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    let mut label = String::new();

    for _ in 0..256 {
        let lens_box: Vec<Lens> = Vec::new();
        boxes.push(lens_box);
    }

    for c in input.bytes() {
        if c == b',' {
            update_boxes(&mut boxes, focal_length, label, index, current);
            current = 0;
            focal_length = 0;
            index += 1;
            label = String::new();
        } else if c >= b'a' && c <= b'z' {
            current += c as usize;
            current *= 17;
            current %= 256;
            label.push(c as char);
        } else if c >= b'0' && c <= b'9' {
            focal_length = c - b'0';
        }
    }
    update_boxes(&mut boxes, focal_length, label, index, current);

    let mut total = 0;
    for i in 0..256 {
        boxes[i].sort_by_key(|a| a.get_index());
        for j in 0..boxes[i].len() {
            total += (i + 1) * (j + 1) * boxes[i][j].focal_length as usize;
        }
    }
    println!("Part 2 soln: {total}");
}

fn update_boxes(
    boxes: &mut Vec<Vec<Lens>>,
    focal_length: u8,
    label: String,
    index: usize,
    current: usize,
) {
    let lens = Lens {
        label: label,
        focal_length: focal_length,
        index: index,
    };
    let find = boxes[current].binary_search_by(|probe| probe.cmp(&lens));
    match find {
        Ok(i) => {
            if focal_length > 0 {
                boxes[current][i].focal_length = focal_length;
            } else {
                boxes[current].remove(i);
            }
        }
        Err(_) => {
            if focal_length > 0 {
                boxes[current].push(lens);
                boxes[current].sort_by(|a, b| a.cmp(b));
            }
        }
    }
}
