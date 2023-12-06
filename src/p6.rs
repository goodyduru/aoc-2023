use std::{fs, u64::MAX};

#[derive(Debug)]
struct Map {
    time: u64,
    distance: u64,
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p6.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return
    };
    first_puzzle(&input);
    second_puzzle(&input);
}

fn first_puzzle(input: &String) {
    let mut digit = 0;
    let mut is_time = true;
    let mut index = 0;
    let mut maps: Vec<Map> = Vec::new();
    for c in input.chars() {
        if c == '\n' {
            let map = Map{time: digit, distance: 0};
            maps.push(map);
            digit = 0;
            is_time = false;
            index = 0;
        }
        if !c.is_digit(10) && digit > 0 {
            if is_time {
                let map = Map{time: digit, distance: 0};
                maps.push(map);
            } else {
                maps[index].distance = digit;
            }
            index += 1;
            digit = 0;
        } else if c.is_digit(10) { 
            digit = digit*10 + c.to_digit(10).unwrap() as u64;
        }
    }
    maps[index].distance = digit;
    
    let mut mul = 1;
    for map in maps {
        mul *= search(&map);
    }
    println!("First puzzle: {mul}");
}

fn search(map: &Map) -> u64 {
    let mut low = 0;
    let mut high = map.time/2;
    let mut mid = 0;
    // Get lower bound
    let mut dist = MAX;
    let mut adjacent_dist = MAX;
    while !(dist > map.distance && adjacent_dist <= map.distance) {
        mid = (low + high) / 2;
        dist = mid*(map.time-mid);
        adjacent_dist = (mid-1)*(map.time-(mid-1));
        if adjacent_dist > map.distance && dist > map.distance {
            high = mid-1;
        } else if adjacent_dist < map.distance && dist < map.distance {
            low = mid + 1;
        }
    }
    let lower_bound = mid;

    // Get higher bound
    high = map.time;
    low = (map.time/2)+1;
    adjacent_dist = MAX;
    dist = MAX;
    while !(dist > map.distance && adjacent_dist <= map.distance) {
        mid = (low + high) / 2;
        dist = mid*(map.time-mid);
        adjacent_dist = (mid+1)*(map.time-(mid+1));
        if adjacent_dist > map.distance && dist > map.distance {
            low = mid+1;
        } else if adjacent_dist < map.distance && dist < map.distance {
            high = mid-1;
        }
    }
    let higher_bound = mid;
    higher_bound - lower_bound + 1
}

fn second_puzzle(input: &String) {
    let mut digit: u64 = 0;
    let mut map = Map { time: 0, distance: 0 };
    for c in input.chars() {
        if c == '\n' {
            map.time = digit;
            digit = 0;
        } else if c.is_digit(10) { 
            digit = digit*10 + c.to_digit(10).unwrap() as u64;
        }
    }
    map.distance = digit;
    
    let mul = search(&map);
    println!("Second puzzle: {mul}");
}