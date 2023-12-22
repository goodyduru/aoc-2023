use std::{cmp::Ordering, fs, time::Instant, collections::{HashMap, HashSet, VecDeque}};

#[derive(Debug)]
struct Brick {
    start: (u16, u16, u16),
    end: (u16, u16, u16),
}

impl Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.2.min(self.end.2).cmp(&other.start.2.min(other.end.2))
    }
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p22.txt");
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
    let mut bricks = parse(input);
    let mut supported_by: HashMap<usize, HashSet<i16>> = HashMap::new();
    let mut supports: HashMap<usize, HashSet<i16>> = HashMap::new();
    fall(&mut bricks, &mut supported_by, &mut supports);

    let mut num_supports: HashSet<usize> = {0..bricks.len()}.collect();
    for (_, v) in supported_by.iter() {
        if v.len() == 1 {
            let t = *(v.iter().nth(0).unwrap()) as usize;
            num_supports.remove(&t);
        }
    }
    println!("Puzzle one soln: {}", num_supports.len());
}

fn parse(input: &String) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = input.lines().map(|line| {
        let it = line.split_once('~').unwrap();
        let mut start = it.0.split(',').map(|c| {
            c.parse::<u16>().unwrap()
        });
        let start = (start.next().unwrap(), start.next().unwrap(), start.next().unwrap());
        let mut end = it.1.split(',').map(|c| {
            c.parse::<u16>().unwrap()
        });
        let end = (end.next().unwrap(), end.next().unwrap(), end.next().unwrap());
        Brick {start: start, end: end}
    }).collect();
    bricks.sort_by(|a, b| a.cmp(b));
    bricks
}

fn fall(bricks: &mut Vec<Brick>, supported_by: &mut HashMap<usize, HashSet<i16>>, supports: &mut HashMap<usize, HashSet<i16>>) {
    let mut max_x = 0;
    let mut max_y = 0;

    for b in bricks.iter() {
        max_x = max_x.max(b.start.0.max(b.end.0));
        max_y = max_y.max(b.start.1.max(b.end.1));
    }

    let mut ground: Vec<Vec<(i16, i16)>> = {0..max_y+1}.map(|_| {
        {0..max_x+1}.map(|_| {
            (1, -1)
        }).collect()
    }).collect();

    for i in 0..bricks.len() {
        let x1 = bricks[i].start.0 as usize;
        let y1 = bricks[i].start.1 as usize;
        let x2 = bricks[i].end.0 as usize;
        let y2 = bricks[i].end.1 as usize;
        let mut max_height = 0;
        for y in y1..y2+1 {
            for x in x1..x2+1 {
                if ground[y][x].0 > max_height {
                    supported_by.entry(i).or_default().clear();
                    max_height = ground[y][x].0;
                }

                if ground[y][x].0 == max_height && ground[y][x].1 >= 0 {
                    supported_by.entry(i).or_default().insert(ground[y][x].1);
                }
            }
        }

        for t in supported_by[&i].iter() {
            let s = *t as usize;
            supports.entry(s).or_default().insert(i as i16);
        }

        let height = bricks[i].end.2 - bricks[i].start.2 + 1;
        for y in y1..y2+1 {
            for x in x1..x2+1 {
                ground[y][x] = (max_height+height as i16, i as i16);
            }
        }

        bricks[i].start.2 = max_height as u16;
        bricks[i].end.2 = max_height as u16 + height - 1;
    }
}

fn second_puzzle(input: &String) {
    let mut bricks = parse(input);
    let mut supported_by: HashMap<usize, HashSet<i16>> = HashMap::new();
    let mut supports: HashMap<usize, HashSet<i16>> = HashMap::new();
    fall(&mut bricks, &mut supported_by, &mut supports);
    let mut result = 0;
    for i in 0..bricks.len() {
        let mut stack = VecDeque::new();
        stack.push_back(i);
        let mut current_falling = HashSet::new();
        current_falling.insert(i as i16);
        while let Some(t) = stack.pop_front() {
            if !supports.contains_key(&t) {
                continue;
            }
            for x in supports[&t].iter() {
                let x = (*x) as usize;
                if supported_by[&x].difference(&current_falling).count() == 0 {
                    current_falling.insert(x as i16);
                    stack.push_back(x);
                }
            }
        }
        result += current_falling.len() - 1;
    }
    println!("Puzzle 2 soln: {result}");
}