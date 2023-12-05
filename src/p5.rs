use std::fs;
use std::cmp::Ordering;
use std::i64::MAX;

#[derive(Debug)]
struct EndItem {
    source: i64,
    dest: i64,
    end: i64
}

impl EndItem {
    fn new(source: i64, dest: i64, end: i64) -> Self {
        Self {
            source,
            dest,
            end,
        }
    }

    fn rcmp(&self, other: &Self) -> Ordering {
        if other.source >= self.source && other.source < self.end {
            return Ordering::Equal;
        }
        else if self.source < other.source {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.source.cmp(&other.source)
    }
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p5.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return
    };
    first_puzzle(&input);
    second_puzzle(&input);
}

fn first_puzzle(input: &String) {
    // build seeds
    let (seeds, index) = get_seeds(input);
    // build maps
    let mut start = index;
    let (soil_map, index) = get_map(&input[start..], false);
    start += index;
    let (fertilizer_map, index) = get_map(&input[start..], false);
    start += index;
    let (water_map, index) = get_map(&input[start..], false);
    start += index;
    let (light_map, index) = get_map(&input[start..], false);
    start += index;
    let (temp_map, index) = get_map(&input[start..], false);
    start += index;
    let (humidity_map, index) = get_map(&input[start..], false);
    start += index;
    let (location_map, _) = get_map(&input[start..], true);

    let mut minimum = MAX;
    for seed in seeds {
        let soil = search(&soil_map, seed);
        let fertilizer = search(&fertilizer_map, soil);
        let water = search(&water_map, fertilizer);
        let light = search(&light_map, water);
        let temp = search(&temp_map, light);
        let humidity = search(&humidity_map, temp);
        let loc = search(&location_map, humidity);
        if loc < minimum {
            minimum = loc;
        }
    }
    println!("First puzzle solution: {minimum}")
}

fn get_seeds(input: &String) -> (Vec<i64>, usize) {
    let mut index = 0;
    let mut seeds: Vec<i64> = Vec::new();
    let mut digit: i64 = 0;
    for c in input.chars() {
        if c == '\n' {
            seeds.push(digit);
            break;
        }
        if !c.is_digit(10) && digit > 0 {
            seeds.push(digit);
            digit = 0;
        } else if c.is_digit(10) { 
            digit = digit*10 + c.to_digit(10).unwrap() as i64;
        }
        index += 1;
    }
    (seeds, index+1)
}

fn get_map(input: &str, final_map: bool) -> (Vec<EndItem>, usize) {
    let mut range: i64;
    let mut source: i64 = -1;
    let mut dest: i64 = -1;
    let mut digit: i64 = -1;
    let mut prev = ' ';
    let mut index = 0;
    let mut soil_map: Vec<EndItem> = Vec::new();
    for c in input.chars() {
        if c == '\n' && prev == '\n' {
            break;
        } else if c == '\n' && dest >= 0 {
            range = digit;
            let item = EndItem::new(source, dest, source+range);
            soil_map.push(item);
            source = -1;
            dest = -1;
            digit = -1;
        } if !c.is_digit(10) && digit >= 0 {
            if dest >= 0 {
                source = digit;
            } else {
                dest = digit;
            }
            digit = -1;
        } else if c.is_digit(10) { 
            digit = if digit > 0 {digit} else {0};
            digit = digit*10 + c.to_digit(10).unwrap() as i64;
        }
        prev = c;
        index += 1;
    }
    if final_map {
        range = digit;
        let item = EndItem::new(source, dest, source+range);
        soil_map.push(item);
    }
    soil_map.sort_by(|a, b| a.cmp(b));
    (soil_map, index+1)
}

fn search(haystack: &Vec<EndItem>, needle: i64) -> i64 {
    let item = EndItem::new(needle, 0, 0);
    let find = haystack.binary_search_by(|probe| probe.rcmp(&item));

    match find {
        Ok(i) => {
            let found = &haystack[i];
            return found.dest + (needle-found.source);
        },
        Err(_) => {
            return needle;
        } 
    }
}

fn second_puzzle(input: &String) {
    // build seeds
    let (seeds, index) = get_item_seeds(input);
    // build maps
    let mut start = index;
    let (soil_map, index) = get_map(&input[start..], false);
    start += index;
    let (fertilizer_map, index) = get_map(&input[start..], false);
    start += index;
    let (water_map, index) = get_map(&input[start..], false);
    start += index;
    let (light_map, index) = get_map(&input[start..], false);
    start += index;
    let (temp_map, index) = get_map(&input[start..], false);
    start += index;
    let (humidity_map, index) = get_map(&input[start..], false);
    start += index;
    let (location_map, _) = get_map(&input[start..], true);
    let maps = vec![soil_map, fertilizer_map, water_map, light_map, temp_map, humidity_map, location_map];

    let mut minimum = MAX;
    for seed in seeds {
        let loc = search_recursive(&seed, &maps, 0);
        if loc < minimum {
            minimum = loc;
        }
    }
    println!("Second puzzle solution: {minimum}")
}

fn get_item_seeds(input: &String) -> (Vec<EndItem>, usize) {
    let mut index = 0;
    let mut seeds: Vec<EndItem> = Vec::new();
    let mut digit: i64 = -1;
    let mut source: i64 = -1;
    for c in input.chars() {
        if c == '\n' {
            seeds.push(EndItem::new(source, 0, source+digit));
            break;
        }
        if !c.is_digit(10) && digit >= 0 {
            if source == -1 {
                source = digit;
            } else {
                seeds.push(EndItem::new(source, 0, source+digit));
                source = -1;
            }
            digit = -1;
        } else if c.is_digit(10) { 
            digit = if digit < 0 {0} else {digit};
            digit = digit*10 + c.to_digit(10).unwrap() as i64;
        }
        index += 1;
    }
    (seeds, index+1)
}

fn search_recursive(item: &EndItem, haystacks: &Vec<Vec<EndItem>>, index: usize) -> i64 {
    let haystack = &haystacks[index];
    let mut minimum = MAX;
    let mut other_item = EndItem::new(item.source, item.dest, item.end);
    let mut new_item: EndItem;
    let mut source = item.source;
    let end = item.end;
    while source < end {
        other_item.source = source;
        other_item.end = end;
        let find = haystack.binary_search_by(|probe| probe.rcmp(&other_item));
        match find {
            Ok(i) => {
                let found = &haystack[i];
                let diff = source - found.source;
                let dest_diff = found.source - found.dest;
                let min_end = if end > found.end { found.end } else { end };
                let loc: i64;
                if index == haystacks.len() - 1 {
                    loc = found.dest + diff;
                    if loc < minimum {
                        minimum = loc;
                    }
                } else {
                    new_item = EndItem::new(found.dest + diff, 0, min_end-dest_diff);
                    loc = search_recursive(&new_item, haystacks, index+1);
                    if loc < minimum {
                        minimum = loc;
                    }
                }
                source = min_end;
            },
            Err(j) => {
                let loc: i64;
                if j >= haystack.len() {
                    if index == haystacks.len() - 1 {
                        loc = source;
                    }
                    else {
                        new_item = EndItem::new(other_item.source, 0, other_item.end);
                        loc = search_recursive(&new_item, haystacks, index+1);
                    }
                    if loc < minimum {
                        minimum = loc;
                    }
                    source = end;
                } else {
                    let intersection;
                    let supposed = &haystack[j];
                    if other_item.end >= supposed.source {
                        intersection = supposed.source;
                    } else {
                        intersection = end;
                    }
                    if index == haystacks.len() - 1 {
                        loc = source;
                    }
                    else {
                        new_item = EndItem::new(other_item.source, 0, intersection);
                        loc = search_recursive(&new_item, haystacks, index+1);
                    }
                    if loc < minimum {
                        minimum = loc;
                    }
                    source = intersection;
                }
            } 
        }
    }
    minimum
}