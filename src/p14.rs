use std::{collections::HashMap, fs, time::Instant};

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p14.txt");
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
    let mut line: Vec<u8> = Vec::new();
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for c in input.bytes() {
        if c == b'\n' {
            grid.push(line);
            line = Vec::new();
        } else {
            line.push(c);
        }
    }
    grid.push(line);
    move_north(&mut grid);
    println!("First puzzle: {}", count_grid(&grid));
}

fn second_puzzle(input: &String) {
    let mut line: Vec<u8> = Vec::new();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut scores: HashMap<(usize, usize, usize, usize), i32> = HashMap::new();

    for c in input.bytes() {
        if c == b'\n' {
            grid.push(line);
            line = Vec::new();
        } else {
            line.push(c);
        }
    }
    grid.push(line);
    let mut almost_there = false;
    let mut count = 0;
    let end = 1000000000;

    loop {
        move_north(&mut grid);
        let north = count_grid(&grid);
        move_west(&mut grid);
        let west = count_grid(&grid);
        move_south(&mut grid);
        let south = count_grid(&grid);
        move_east(&mut grid);
        let east = count_grid(&grid);
        count += 1;
        if !almost_there && scores.contains_key(&(north, west, south, east)) {
            let index = scores.get(&(north, west, south, east)).unwrap();
            let period = count - index;
            count = end - ((end - index) % period);
            almost_there = true;
        }
        scores.insert((north, west, south, east), count);

        if count == end {
            println!("Second puzzle soln: {east}");
            break;
        }
    }
}

fn count_grid(grid: &Vec<Vec<u8>>) -> usize {
    let len = grid.len();
    let mut total = 0;
    for (i, l) in grid.iter().enumerate() {
        let mut count = 0;
        for c in l {
            if *c == b'O' {
                count += len - i;
            }
        }
        total += count;
    }
    total
}

fn move_north(grid: &mut Vec<Vec<u8>>) {
    let mut spaces: Vec<i8> = Vec::new();
    for b in &grid[0] {
        if *b == b'.' {
            spaces.push(0);
        } else {
            spaces.push(-1);
        }
    }

    for i in 1..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'.' && spaces[j] == -1 {
                spaces[j] = i as i8;
            } else if grid[i][j] == b'#' {
                spaces[j] = -1;
            } else if grid[i][j] == b'O' && spaces[j] > -1 {
                grid[spaces[j] as usize][j] = b'O';
                grid[i][j] = b'.';
                spaces[j] += 1;
            }
        }
    }
}

fn move_west(grid: &mut Vec<Vec<u8>>) {
    let len = grid[0].len();
    for i in 0..grid.len() {
        let mut last_space: i8 = -1;
        for j in 0..len {
            if grid[i][j] == b'.' && last_space == -1 {
                last_space = j as i8;
            } else if grid[i][j] == b'#' {
                last_space = -1;
            } else if grid[i][j] == b'O' && last_space > -1 {
                grid[i][last_space as usize] = b'O';
                grid[i][j] = b'.';
                last_space += 1;
            }
        }
    }
}

fn move_south(grid: &mut Vec<Vec<u8>>) {
    let mut spaces: Vec<i8> = Vec::new();
    let end = grid.len() - 1;
    for b in &grid[end] {
        if *b == b'.' {
            spaces.push(end as i8);
        } else {
            spaces.push(-1);
        }
    }

    for i in { 0..end }.rev() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'.' && spaces[j] == -1 {
                spaces[j] = i as i8;
            } else if grid[i][j] == b'#' {
                spaces[j] = -1;
            } else if grid[i][j] == b'O' && spaces[j] > -1 {
                grid[spaces[j] as usize][j] = b'O';
                grid[i][j] = b'.';
                spaces[j] -= 1;
            }
        }
    }
}

fn move_east(grid: &mut Vec<Vec<u8>>) {
    let len = grid[0].len() - 1;
    for i in 0..grid.len() {
        let mut last_space: i8 = -1;
        for j in { 0..=len }.rev() {
            if grid[i][j] == b'.' && last_space == -1 {
                last_space = j as i8;
            } else if grid[i][j] == b'#' {
                last_space = -1;
            } else if grid[i][j] == b'O' && last_space > -1 {
                grid[i][last_space as usize] = b'O';
                grid[i][j] = b'.';
                last_space -= 1;
            }
        }
    }
}
