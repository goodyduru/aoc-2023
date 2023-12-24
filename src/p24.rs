use std::{cmp::Ordering, collections::HashSet, fs, time::Instant};

#[derive(Debug)]
struct Stone {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

impl Stone {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.0.cmp(&other.position.0)
    }
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p24.txt");
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
    let stones = parse(input);
    let test_area = (200000000000000.0, 400000000000000.0);
    let mut answer = 0;
    for i in 0..stones.len() {
        for j in i + 1..stones.len() {
            answer += cramer(&stones[i], &stones[j], &test_area);
        }
    }
    println!("Puzzle one: {answer}");
}

fn parse(input: &String) -> Vec<Stone> {
    let stones: Vec<Stone> = input
        .lines()
        .map(|line| {
            let it = line.split_once(" @ ").unwrap();
            let mut position = it.0.split(", ").map(|c| c.parse::<isize>().unwrap());
            let position = (
                position.next().unwrap(),
                position.next().unwrap(),
                position.next().unwrap(),
            );
            let mut velocity =
                it.1.split(", ")
                    .map(|c| c.trim_start_matches(' ').parse::<isize>().unwrap());
            let velocity = (
                velocity.next().unwrap(),
                velocity.next().unwrap(),
                velocity.next().unwrap(),
            );
            Stone { position, velocity }
        })
        .collect();
    stones
}

fn cramer(first: &Stone, second: &Stone, test_area: &(f64, f64)) -> usize {
    let delta = (first.velocity.0 * (-1 * second.velocity.1))
        - ((-1 * second.velocity.0) * first.velocity.1);
    if delta == 0 {
        return 0;
    }
    let diff_x = (first.position.0 - second.position.0) * -1;
    let diff_y = (first.position.1 - second.position.1) * -1;
    let num_x = ((diff_x * (-1 * second.velocity.1)) - ((-1 * second.velocity.0) * diff_y)) as f64;
    let delta = delta as f64;
    let predict_x = first.position.0 as f64 + (first.velocity.0 as f64 * num_x) / delta;
    let predict_y = first.position.1 as f64 + (first.velocity.1 as f64 * num_x) / delta;
    if (predict_x >= first.position.0 as f64 && first.velocity.0 < 0)
        || (predict_x <= first.position.0 as f64 && first.velocity.0 > 0)
        || (predict_y >= first.position.1 as f64 && first.velocity.1 < 0)
        || (predict_y <= first.position.1 as f64 && first.velocity.1 > 0)
        || (predict_x >= second.position.0 as f64 && second.velocity.0 < 0)
        || (predict_x <= second.position.0 as f64 && second.velocity.0 > 0)
        || (predict_y >= second.position.1 as f64 && second.velocity.1 < 0)
        || (predict_y <= second.position.1 as f64 && second.velocity.1 > 0)
    {
        return 0;
    }
    if predict_x < test_area.0
        || predict_x > test_area.1
        || predict_y < test_area.0
        || predict_y > test_area.1
    {
        return 0;
    }
    1
}

fn second_puzzle(input: &String) {
    let mut stones = parse(input);
    stones.sort_by(|a, b| a.cmp(b));
    let mut answer = 0;
    let mut x_set = HashSet::new();
    let mut y_set = HashSet::new();
    let mut z_set = HashSet::new();
    for i in 0..stones.len() {
        for j in i + 1..stones.len() {
            let sets = inter(&stones[i], &stones[j]);
            if x_set.len() == 0 {
                x_set.extend(sets.0);
            } else if sets.0.len() > 0 {
                x_set.retain(|k| sets.0.contains(k));
            }

            if y_set.len() == 0 {
                y_set.extend(sets.1);
            } else if sets.1.len() > 0 {
                y_set.retain(|k| sets.1.contains(k));
            }

            if z_set.len() == 0 {
                z_set.extend(sets.2);
            } else if sets.2.len() > 0 {
                z_set.retain(|k| sets.2.contains(k));
            }
        }
    }
    let x = x_set.iter().next().unwrap();
    let y = y_set.iter().next().unwrap();
    let z = z_set.iter().next().unwrap();
    let m_a = (stones[0].velocity.1 - y) as f64 / (stones[0].velocity.0 - x) as f64;
    let m_b = (stones[1].velocity.1 - y) as f64 / (stones[1].velocity.0 - x) as f64;
    let c_a = stones[0].position.1 as f64 - (m_a * stones[0].position.0 as f64);
    let c_b = stones[1].position.1 as f64 - (m_b * stones[1].position.0 as f64);
    let xpos = ((c_b - c_a) / (m_a - m_b)) as isize;
    let ypos = (m_a * xpos as f64 + c_a) as isize;
    let time = (xpos - stones[0].position.0) / (stones[0].velocity.0 - x);
    let zpos = stones[0].position.2 + (stones[0].velocity.2 - z) * time;
    answer = xpos + ypos + zpos;
    println!("Puzzle two: {answer}");
}

fn inter(first: &Stone, second: &Stone) -> (HashSet<isize>, HashSet<isize>, HashSet<isize>) {
    let mut new_x = HashSet::new();
    if first.velocity.0 == second.velocity.0 && first.velocity.0.abs() > 100 {
        let diff = second.position.0 - first.position.0;
        for v in -1000..1000 {
            if v == first.velocity.0 {
                continue;
            }
            if diff % (v - first.velocity.0) == 0 {
                new_x.insert(v);
            }
        }
    }

    let mut new_y = HashSet::new();
    if first.velocity.1 == second.velocity.1 && first.velocity.1.abs() > 100 {
        let diff = second.position.1 - first.position.1;
        for v in -1000..1000 {
            if v == first.velocity.1 {
                continue;
            }
            if diff % (v - first.velocity.1) == 0 {
                new_y.insert(v);
            }
        }
    }

    let mut new_z = HashSet::new();
    if first.velocity.2 == second.velocity.2 && first.velocity.2.abs() > 100 {
        let diff = second.position.2 - first.position.2;
        for v in -1000..1000 {
            if v == first.velocity.2 {
                continue;
            }
            if diff % (v - first.velocity.2) == 0 {
                new_z.insert(v);
            }
        }
    }

    (new_x, new_y, new_z)
}
