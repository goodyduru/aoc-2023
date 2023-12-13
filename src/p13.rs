use std::{fs, time::Instant};

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p13.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return,
    };
    let now = Instant::now();
    puzzle(&input, false);
    puzzle(&input, true);
    let elapsed = Instant::elapsed(&now);
    println!("{:?}", elapsed);
}

fn puzzle(input: &String, smudge: bool) {
    let mut prev_char = ' ';
    let mut line = String::new();
    let mut lines: Vec<String> = Vec::new();
    let mut sum = 0;
    for c in input.chars() {
        if c == '\n' && prev_char == '\n' {
            sum += mirror(&lines, smudge);
            lines.clear();
            line = String::new();
        } else if c == '\n' {
            lines.push(line);
            line = String::new();
        } else {
            line.push(c);
        }
        prev_char = c;
    }
    lines.push(line);
    sum += mirror(&lines, smudge);

    println!("Puzzle soln: {sum}");
}

fn mirror(lines: &Vec<String>, smudge: bool) -> isize {
    let max_diff: u32;
    if smudge {
        max_diff = 1;
    } else {
        max_diff = 0;
    }
    let mut f_diff;
    // Find reflections across rows
    for i in 0..lines.len() - 1 {
        f_diff = 0;
        if custom_compare(&lines[i], &lines[i + 1], max_diff) <= max_diff {
            let mut j = i as isize;
            let mut k = i + 1;
            while j >= 0 && k < lines.len() {
                let diff = custom_compare(&lines[j as usize], &lines[k as usize], max_diff);
                f_diff += diff;
                if f_diff > max_diff {
                    break;
                }
                j -= 1;
                k += 1;
            }
            if (j < 0 || k == lines.len()) && f_diff == max_diff {
                return 100 * (i + 1) as isize;
            }
        }
    }
    let mut cols: Vec<String> = Vec::with_capacity(lines[0].len());
    for _ in 0..lines[0].len() {
        cols.push(String::with_capacity(lines.len()));
    }

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            cols[i].push(c);
        }
    }

    // Find reflections across columns
    for i in 0..cols.len() - 1 {
        f_diff = 0;
        if custom_compare(&cols[i], &cols[i + 1], max_diff) <= max_diff {
            let mut j = i as isize;
            let mut k = i + 1;
            while j >= 0 && k < cols.len() {
                let diff = custom_compare(&cols[j as usize], &cols[k as usize], max_diff);
                f_diff += diff;
                if f_diff > max_diff {
                    break;
                }
                j -= 1;
                k += 1;
            }
            if (j < 0 || k == cols.len()) && f_diff == max_diff {
                return 1 + i as isize;
            }
        }
    }
    0
}

fn custom_compare(s1: &String, s2: &String, maxdiff: u32) -> u32 {
    let mut diff: u32 = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            diff += 1;
        }
        if diff > maxdiff {
            break;
        }
    }
    diff
}
