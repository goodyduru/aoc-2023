use std::{
    collections::{BinaryHeap, HashMap},
    fs,
    time::Instant,
    usize::MAX,
};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
    from: [u8; 4],
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    N,
    W,
    S,
    E,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::W => Direction::E,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
        }
    }
}

struct Node {
    value: u8,
    neighbors: [isize; 4], // up, left, down, right
}

impl Node {
    fn new(value: u8) -> Self {
        Self {
            value,
            neighbors: [0; 4],
        }
    }
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/test.txt");
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
    let mut max_col = 0;
    let mut line_number: isize = 0;
    let mut index = 0;
    let mut nodes: Vec<Node> = Vec::new();

    for c in input.bytes() {
        if c == b'\n' {
            if line_number == 0 {
                max_col = index;
            }
            index = 0;
            line_number += 1;
        } else {
            nodes.push(Node::new(c - b'0'));
            index += 1;
        }
    }
    line_number += 1;

    set_neighbors(&mut nodes, max_col, line_number);
    let last_index = nodes.len() - 1;
    let ans = shortest_distance(&nodes, 0, last_index, 0, 3);
    println!("First puzzle soln: {}", ans);
}

fn set_neighbors(nodes: &mut Vec<Node>, max_col: u32, line_number: isize) {
    for i in 0..nodes.len() {
        let j = i as isize;
        let row = j / line_number;
        let col = j % line_number;
        let up = (row - 1) * line_number + col;
        let down = (row + 1) * line_number + col;
        let right = row * line_number + (col + 1);
        let left = row * line_number + (col - 1);
        nodes[i].neighbors[0] = if row > 0 { up } else { -1 };
        nodes[i].neighbors[1] = if col > 0 { left } else { -1 };
        nodes[i].neighbors[2] = if row + 1 < line_number { down } else { -1 };
        nodes[i].neighbors[3] = if col + 1 < max_col as isize {
            right
        } else {
            -1
        };
    }
}

fn second_puzzle(input: &String) {
    let mut max_col = 0;
    let mut line_number: isize = 0;
    let mut index = 0;
    let mut nodes: Vec<Node> = Vec::new();

    for c in input.bytes() {
        if c == b'\n' {
            if line_number == 0 {
                max_col = index;
            }
            index = 0;
            line_number += 1;
        } else {
            nodes.push(Node::new(c - b'0'));
            index += 1;
        }
    }
    line_number += 1;

    set_neighbors(&mut nodes, max_col, line_number);
    let last_index = nodes.len() - 1;
    let ans = shortest_distance(&nodes, 0, last_index, 4, 10);
    println!("Second puzzle soln: {}", ans);
}

fn shortest_distance(
    nodes: &Vec<Node>,
    start: usize,
    goal: usize,
    min_steps: u8,
    max_steps: u8,
) -> usize {
    let mut distance: HashMap<(usize, Direction, u8), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distance.insert((start, Direction::S, 0), 0);
    distance.insert((start, Direction::E, 0), 0);
    heap.push(State {
        cost: 0,
        position: start,
        from: [0; 4],
        dir: Direction::E,
    });
    while let Some(State {
        cost,
        position,
        from,
        dir,
    }) = heap.pop()
    {
        let t = match dir {
            Direction::N => from[0],
            Direction::W => from[1],
            Direction::S => from[2],
            Direction::E => from[3],
        };
        if position == goal && t >= min_steps {
            return cost;
        }
        let key = (position, dir, t);
        if distance.contains_key(&key) && cost > distance[&key] {
            continue;
        }
        let neighbors = nodes[position].neighbors;
        let max = from.iter().max().unwrap_or(&0);
        for i in 0..4 {
            if neighbors[i] == -1 || from[i] == max_steps {
                continue;
            }
            let j: Direction;
            if i == 0 {
                j = Direction::N;
            } else if i == 1 {
                j = Direction::W;
            } else if i == 2 {
                j = Direction::S;
            } else {
                j = Direction::E;
            }
            if min_steps == 0 && j == dir.opposite() {
                continue;
            }
            if min_steps > 0 && *max < min_steps && j != dir {
                continue;
            }
            let index = neighbors[i] as usize;
            let mut next_from = [0, 0, 0, 0];
            next_from[i] = from[i] + 1;
            let next = State {
                cost: cost + nodes[index].value as usize,
                position: index,
                from: next_from,
                dir: j,
            };
            let key = (index, j, next_from[i]);
            if !distance.contains_key(&key) || next.cost < distance[&key] {
                distance.insert(key, next.cost);
                heap.push(next);
            }
        }
    }
    MAX
}
