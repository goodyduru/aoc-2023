use std::collections::{BinaryHeap, HashSet};
use std::{cmp::Reverse, fs, time::Instant};

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p21.txt");
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
    let mut starter = (0, 0, 0);
    let mut grid = Vec::new();
    input.lines().for_each(|line| {
        let mut row = Vec::new();
        line.bytes().for_each(|byte| {
            if byte == b'S' {
                starter = (0, row.len() as i32, grid.len() as i32);
                row.push(b'.');
            } else {
                row.push(byte);
            }
        });
        grid.push(row);
    });
    let answer = search(&grid, 64, starter, false);
    println!("Puzzle one soln: {answer}");
}

fn search(grid: &Vec<Vec<u8>>, target: i32, starter: (i32, i32, i32), can_expand: bool) -> i32 {
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(starter));
    let mut answer = 0; 
    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;
    while let Some(Reverse(pos)) = queue.pop() {
        let (steps, x, y) = pos;
        if steps > target {
            break;
        }
        if seen.contains(&(x, y)) {
            continue;
        }
        if steps == target || (target % 2) == (steps % 2) {
            answer += 1;
            if steps == target {
                seen.insert((x, y));
                continue;
            }
        }
        let left = (steps+1, x-1, y);
        let right = (steps+1, x+1, y);
        let up = (steps+1, x, y-1);
        let down = (steps+1, x, y+1);
        let mut new_x: i32;
        let mut new_y: i32;
        if can_expand {
            new_x =  left.1.rem_euclid(max_x);
            new_y = left.2.rem_euclid(max_y);
        } else {
            new_x =  left.1;
            new_y = left.2;
        }
        if (can_expand || (!can_expand && new_x >= 0 )) && grid[new_y as usize][new_x as usize] == b'.' && !seen.contains(&(left.1, left.2)) {
            queue.push(Reverse(left));
        }

        if can_expand {
            new_x =  right.1.rem_euclid(max_x);
            new_y = right.2.rem_euclid(max_y);
        } else {
            new_x =  right.1;
            new_y = right.2;
        }
        if (can_expand || (!can_expand && new_x < max_x )) && grid[new_y as usize][new_x as usize] == b'.' && !seen.contains(&(right.1, right.2)) {
            queue.push(Reverse(right));
        }
        if can_expand {
            new_x =  up.1.rem_euclid(max_x);
            new_y = up.2.rem_euclid(max_y);
        } else {
            new_x =  up.1;
            new_y = up.2;
        }
        if (can_expand || (!can_expand && new_y >= 0 )) && grid[new_y as usize][new_x as usize] == b'.' && !seen.contains(&(up.1, up.2)) {
            queue.push(Reverse(up));
        }
        if can_expand {
            new_x =  down.1.rem_euclid(max_x);
            new_y = down.2.rem_euclid(max_y);
        } else {
            new_x =  down.1;
            new_y = down.2;
        }
        if (can_expand || (!can_expand && new_y < max_y )) && grid[new_y as usize][new_x as usize] == b'.' && !seen.contains(&(down.1, down.2)) {
            queue.push(Reverse(down));
        }
        seen.insert((x, y));
    }
    answer
}

fn second_puzzle(input: &String) {
    let mut starter = (0, 0, 0);
    let mut grid = Vec::new();
    input.lines().for_each(|line| {
        let mut row = Vec::new();
        line.bytes().for_each(|byte| {
            if byte == b'S' {
                starter = (0, row.len() as i32, grid.len() as i32);
                row.push(b'.');
            } else {
                row.push(byte);
            }
        });
        grid.push(row);
    });

    let half = (grid.len()/2) as i32;
    let size = grid.len() as i32;
    let targets = [half, half+size, half+size*2];

    let answers: Vec<i32> = targets.iter().map(|target| {
        let s = starter.clone();
        let answer = search(&grid, *target, s, true);
        answer
    }).collect();
    let a = ((answers[2] + answers[0] - 2*answers[1]) / 2) as u64;
    let b = (answers[1] - answers[0] - a as i32) as u64;
    let c = (answers[0]) as u64;
    let n = (26501365 / size) as u64;
    let result = a*n*n + b*n + c;
    println!("Puzzle 2 soln: {result}") ;
}