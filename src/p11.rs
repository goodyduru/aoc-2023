use std::{cmp::Ordering, collections::HashMap, fs, time::Instant};

#[derive(Debug)]
struct Galaxy {
    row: i64,
    col: i64,
}

impl Galaxy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.col.cmp(&other.col)
    }
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p11.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return,
    };
    let now = Instant::now();
    puzzle(&input, 1);
    puzzle(&input, 999999);
    let elapsed = Instant::elapsed(&now);
    println!("{:?}", elapsed);
}

fn puzzle(input: &String, expansion_factor: i64) {
    let mut index = 0;
    let mut line_num = 0;
    let mut empty_row = true;
    let mut cols: HashMap<i64, bool> = HashMap::new();
    let mut empty_rows: Vec<i64> = Vec::new();
    let mut empty_cols: Vec<i64> = Vec::new();
    let mut galaxies: Vec<Galaxy> = Vec::new();

    for c in input.chars() {
        if c == '\n' {
            if empty_row {
                empty_rows.push(line_num);
            }
            index = 0;
            empty_row = true;
            line_num += 1;
        } else if c == '#' {
            empty_row = false;
            cols.insert(index, false);
            galaxies.push(Galaxy {
                row: line_num,
                col: index,
            });
            index += 1;
        } else {
            if line_num == 0 {
                cols.insert(index, true);
            }
            index += 1;
        }
    }

    for (k, _) in cols.iter().filter(|&(_, v)| *v == true) {
        empty_cols.push(*k);
    }

    empty_cols.sort();
    empty_cols.reverse();

    empty_rows.reverse();

    galaxies.reverse();

    let mut i = 0;
    let mut j = 0;
    let col_len = empty_cols.len();
    let row_len = empty_rows.len();
    let gal_len = galaxies.len();
    while i < row_len {
        while j < gal_len {
            if galaxies[j].row < empty_rows[i] {
                break;
            }
            galaxies[j].row += (row_len - i) as i64 * expansion_factor;
            j += 1;
        }
        i += 1;
    }

    i = 0;
    j = 0;
    galaxies.sort_by(|a, b| b.cmp(a));
    while i < col_len {
        while j < gal_len {
            if galaxies[j].col < empty_cols[i] {
                break;
            }
            galaxies[j].col += (col_len - i) as i64 * expansion_factor;
            j += 1;
        }
        i += 1;
    }
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            sum += (galaxies[i].row - galaxies[j].row).abs()
                + (galaxies[i].col - galaxies[j].col).abs()
        }
    }

    println!("Puzzle sum: {sum}");
}
