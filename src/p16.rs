use std::{collections::HashSet, fs, time::Instant};

struct Beam {
    current: (i32, i32),
    next: (i32, i32),
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p16.txt");
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
    let beam = Beam {
        current: (0, 0),
        next: (1, 0),
    };
    let total = energize(&grid, beam);

    println!("First puzzle soln: {total}");
}

fn energize(grid: &Vec<Vec<u8>>, beam: Beam) -> usize {
    let mut beams: Vec<Beam> = Vec::new();
    let mut energize: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let mut hashes: HashSet<(i32, i32)> = HashSet::new();
    beams.push(beam);
    loop {
        while beams[0].current.0 >= 0
            && beams[0].current.0 < grid[0].len() as i32
            && beams[0].current.1 >= 0
            && beams[0].current.1 < grid.len() as i32
        {
            let next = (
                beams[0].current.0,
                beams[0].current.1,
                beams[0].current.0 + beams[0].next.0,
                beams[0].current.1 + beams[0].next.1,
            );
            if energize.contains(&next) {
                break;
            }
            hashes.insert((next.0, next.1));
            let c = grid[next.1 as usize][next.0 as usize];
            if c == b'.' {
                beams[0].current.0 = next.2;
                beams[0].current.1 = next.3;
            } else if c == b'/' {
                if beams[0].next == (1, 0) {
                    beams[0].next = (0, -1);
                    beams[0].current = (beams[0].current.0, beams[0].current.1 - 1);
                } else if beams[0].next == (-1, 0) {
                    beams[0].next = (0, 1);
                    beams[0].current = (beams[0].current.0, beams[0].current.1 + 1);
                } else if beams[0].next == (0, 1) {
                    beams[0].next = (-1, 0);
                    beams[0].current = (beams[0].current.0 - 1, beams[0].current.1);
                } else {
                    beams[0].next = (1, 0);
                    beams[0].current = (beams[0].current.0 + 1, beams[0].current.1);
                }
            } else if c == b'\\' {
                if beams[0].next == (1, 0) {
                    beams[0].next = (0, 1);
                    beams[0].current = (beams[0].current.0, beams[0].current.1 + 1);
                } else if beams[0].next == (-1, 0) {
                    beams[0].next = (0, -1);
                    beams[0].current = (beams[0].current.0, beams[0].current.1 - 1);
                } else if beams[0].next == (0, 1) {
                    beams[0].next = (1, 0);
                    beams[0].current = (beams[0].current.0 + 1, beams[0].current.1);
                } else {
                    beams[0].next = (-1, 0);
                    beams[0].current = (beams[0].current.0 - 1, beams[0].current.1);
                }
            } else if c == b'-' {
                if beams[0].next.0 != 0 {
                    beams[0].current.0 = next.2;
                    beams[0].current.1 = next.3;
                } else {
                    beams.push(Beam {
                        current: (beams[0].current.0 - 1, beams[0].current.1),
                        next: (-1, 0),
                    });
                    beams[0].next = (1, 0);
                    beams[0].current = (beams[0].current.0 + 1, beams[0].current.1);
                }
            } else {
                if beams[0].next.1 != 0 {
                    beams[0].current.0 = next.2;
                    beams[0].current.1 = next.3;
                } else {
                    beams.push(Beam {
                        current: (beams[0].current.0, beams[0].current.1 - 1),
                        next: (0, -1),
                    });
                    beams[0].next = (0, 1);
                    beams[0].current = (beams[0].current.0, beams[0].current.1 + 1);
                }
            }
            energize.insert(next);
        }
        beams.remove(0);
        if beams.len() == 0 {
            break;
        }
    }
    hashes.len()
}

fn second_puzzle(input: &String) {
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
    let mut max = 0;

    for i in 0..grid.len() {
        let beam = Beam {
            current: (0, i as i32),
            next: (1, 0),
        };
        let total = energize(&grid, beam);
        if total > max {
            max = total
        }

        let beam = Beam {
            current: (grid[0].len() as i32 - 1, i as i32),
            next: (-1, 0),
        };
        let total = energize(&grid, beam);
        if total > max {
            max = total
        }
    }

    for i in 0..grid[0].len() {
        let beam = Beam {
            current: (i as i32, 0),
            next: (0, 1),
        };
        let total = energize(&grid, beam);
        if total > max {
            max = total
        }

        let beam = Beam {
            current: (i as i32, grid.len() as i32 - 1),
            next: (0, -1),
        };
        let total = energize(&grid, beam);
        if total > max {
            max = total
        }
    }

    println!("First puzzle soln: {max}");
}
