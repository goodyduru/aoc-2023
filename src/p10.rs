use std::{fs, i32::MAX, time::Instant};

#[derive(Debug)]
struct Node {
    value: char,
    dist: i32,
    index: i32,
    neighbors: [i32; 2],
}

impl Node {
    fn new(value: char, index: i32) -> Self {
        Self {
            value,
            index,
            dist: MAX,
            neighbors: [0; 2],
        }
    }

    fn set_neighbors(&mut self, length: i32, max_row: i32) {
        let row = self.index / length;
        let col = self.index % length;
        let next_col = if col + 1 == length { -1 } else { col + 1 };
        let next_row = if row + 1 == max_row { -1 } else { row + 1 };
        let prev_col = col - 1;
        let prev_row = row - 1;
        match self.value {
            '|' => {
                self.neighbors[0] = if prev_row > -1 {
                    prev_row * length + col
                } else {
                    prev_row
                };
                self.neighbors[1] = if next_row > -1 {
                    next_row * length + col
                } else {
                    next_row
                };
            }
            '-' => {
                self.neighbors[0] = if prev_col > -1 {
                    row * length + prev_col
                } else {
                    prev_col
                };
                self.neighbors[1] = if next_col > -1 {
                    row * length + next_col
                } else {
                    next_col
                };
            }
            'L' => {
                self.neighbors[0] = if prev_row > -1 {
                    prev_row * length + col
                } else {
                    prev_row
                };
                self.neighbors[1] = if next_col > -1 {
                    row * length + next_col
                } else {
                    next_col
                };
            }
            'J' => {
                self.neighbors[0] = if prev_row > -1 {
                    prev_row * length + col
                } else {
                    prev_row
                };
                self.neighbors[1] = if prev_col > -1 {
                    row * length + prev_col
                } else {
                    prev_col
                };
            }
            '7' => {
                self.neighbors[0] = if next_row > -1 {
                    next_row * length + col
                } else {
                    next_row
                };
                self.neighbors[1] = if prev_col > -1 {
                    row * length + prev_col
                } else {
                    prev_col
                };
            }
            'F' => {
                self.neighbors[0] = if next_col > -1 {
                    row * length + next_col
                } else {
                    next_col
                };
                self.neighbors[1] = if next_row > -1 {
                    next_row * length + col
                } else {
                    next_row
                };
            }
            _ => {}
        }
    }

    fn get_neighbor(&self, prev_index: i32) -> i32 {
        if self.neighbors[0] == -1 && self.neighbors[1] == -1 {
            return -1;
        }
        if prev_index == self.neighbors[0] {
            return self.neighbors[1];
        }

        if prev_index == self.neighbors[1] {
            return self.neighbors[0];
        }
        return -1;
    }

    fn set_dist(&mut self, val: i32) {
        self.dist = val;
    }
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p10.txt");
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
    let mut index = 0;
    let mut vertices: Vec<Node> = Vec::new();
    let mut num_lines = 1;
    let mut max_col = 0;
    let mut start = 0;

    for c in input.chars() {
        if c == '\n' {
            num_lines += 1;
            max_col = if max_col == 0 { index } else { max_col };
        } else {
            vertices.push(Node::new(c, index));
            if c == 'S' {
                start = index;
            }
            index += 1;
        }
    }
    for node in vertices.iter_mut() {
        node.set_neighbors(max_col, num_lines);
    }
    vertices.get_mut(start as usize).unwrap().set_dist(0);
    let index = get_starter(&vertices, start, max_col, num_lines);
    let max_val = transverse(&mut vertices, start, index);
    println!("First puzzle solution is {max_val}");
}

fn get_starter(vertices: &Vec<Node>, start: i32, length: i32, max_row: i32) -> i32 {
    let mut mindex = -1;
    let row = start / length;
    let col = start % length;
    let next_col = if col + 1 == length { -1 } else { col + 1 };
    let next_row = if row + 1 == max_row { -1 } else { row + 1 };
    let prev_col = col - 1;
    let prev_row = row - 1;
    if prev_col > -1 {
        let index = row * length + prev_col;
        let n1 = vertices[index as usize].neighbors[0];
        let n2 = vertices[index as usize].neighbors[1];
        mindex = if n1 == start || n2 == start {
            index
        } else {
            mindex
        };
    }

    if mindex == -1 && next_col > -1 {
        let index = row * length + next_col;
        let n1 = vertices[index as usize].neighbors[0];
        let n2 = vertices[index as usize].neighbors[1];
        mindex = if n1 == start || n2 == start {
            index
        } else {
            mindex
        };
    }

    if mindex > -1 && prev_row > -1 {
        let index = prev_row * length + col;
        let n1 = vertices[index as usize].neighbors[0];
        let n2 = vertices[index as usize].neighbors[1];
        mindex = if n1 == start || n2 == start {
            index
        } else {
            mindex
        };
    }

    if mindex > -1 && next_row > -1 {
        let index = next_row * length + col;
        let n1 = vertices[index as usize].neighbors[0];
        let n2 = vertices[index as usize].neighbors[1];
        mindex = if n1 == start || n2 == start {
            index
        } else {
            mindex
        };
    }
    mindex
}

fn transverse(vertices: &mut Vec<Node>, start: i32, index: i32) -> i32 {
    let mut next: i32;
    let mut dist: i32;
    let mut max = 0;
    let mut n = 0;
    let mut index = index;
    let mut prev = start;

    while index > -1 {
        next = vertices.get(index as usize).unwrap().get_neighbor(prev);
        dist = vertices.get(index as usize).unwrap().dist;
        if next > -1 && dist >= n {
            n += 1;
            vertices.get_mut(index as usize).unwrap().set_dist(n);
            max = if n > max { n } else { max };
            prev = index;
            index = next;
        } else {
            break;
        }
    }
    (max + 1) / 2
}
fn second_puzzle(input: &String) {
    let mut index = 0;
    let mut vertices: Vec<Node> = Vec::new();
    let mut num_lines = 1;
    let mut max_col = 0;
    let mut start = 0;

    for c in input.chars() {
        if c == '\n' {
            num_lines += 1;
            max_col = if max_col == 0 { index } else { max_col };
        } else {
            vertices.push(Node::new(c, index));
            if c == 'S' {
                start = index;
            }
            index += 1;
        }
    }
    for node in vertices.iter_mut() {
        node.set_neighbors(max_col, num_lines);
    }
    println!("{max_col}");
    vertices.get_mut(start as usize).unwrap().set_dist(0);
    let index = get_starter(&vertices, start, max_col, num_lines);
    let max_val = transverse_with_touch(&mut vertices, start, index, max_col);
    println!("Second puzzle solution is {max_val}");
}

fn transverse_with_touch(vertices: &mut Vec<Node>, start: i32, index: i32, max_col: i32) -> i32 {
    let mut next: i32;
    let mut dist: i32;
    let mut max = 0;
    let mut n = 0;
    let mut index = index;
    let mut prev = start;
    let mut bends: Vec<u32> = Vec::new();

    while index > -1 {
        next = vertices.get(index as usize).unwrap().get_neighbor(prev);
        dist = vertices.get(index as usize).unwrap().dist;
        if next > -1 && dist >= n {
            n += 1;
            vertices.get_mut(index as usize).unwrap().set_dist(n);
            let c = vertices[index as usize].value;
            max = if n > max { n } else { max };
            if c == 'J' || c == 'F' || c == '7' || c == 'L' {
                bends.push(index as u32);
            }
            prev = index;
            index = next;
        } else {
            break;
        }
    }
    max += 1;

    // Picks theoerem
    bends.insert(0, start as u32);
    let len = bends.len();
    bends.insert(len, start as u32);
    let mut area: i32 = 0;
    let length = max_col as u32;
    // Shoelace formula
    for i in 0..len {
        let row = bends[i] / length;
        let col = bends[i] % length;
        let next_row = bends[i + 1] / length;
        let next_col = bends[i + 1] % length;
        area += (row * next_col) as i32 - (col * next_row) as i32;
    }
    area / 2 + 1 - max / 2
}
