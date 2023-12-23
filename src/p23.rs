use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{fs, time::Instant};

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p23.txt");
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
    let grid = input.lines().map(|line| line.bytes().collect()).collect();
    let answer = search(&grid);
    println!("Puzzle one soln: {answer}");
}

fn search(grid: &Vec<Vec<u8>>) -> usize {
    let starter = (0, 1, 0, 0);
    let target = (grid[0].len() as isize - 2, grid.len() as isize - 1);
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut answer = 0;
    let mut max_seen = 0;
    queue.push(starter);
    while let Some((steps, x, y, index)) = queue.pop() {
        if (x, y) == target {
            answer = answer.max(steps);
            continue;
        }
        let max_x = grid[0].len() as isize;
        let max_y = grid.len() as isize;
        let left = (steps + 1, x - 1, y, index);
        let mut right = (steps + 1, x + 1, y, index);
        let mut up = (steps + 1, x, y - 1, index);
        let mut down = (steps + 1, x, y + 1, index);
        let mut num_seen = 0;
        if left.1 >= 0
            && grid[y as usize][x as usize] == b'.'
            && grid[left.2 as usize][left.1 as usize] != b'#'
            && !seen.contains(&(left.1, left.2, index))
        {
            queue.push(left);
            num_seen += 1;
        }

        if right.1 < max_x
            && (grid[y as usize][x as usize] == b'.' || grid[y as usize][x as usize] == b'>')
            && grid[right.2 as usize][right.1 as usize] != b'#'
            && !seen.contains(&(right.1, right.2, index))
        {
            if num_seen > 0 {
                max_seen += 1;
                right.3 = max_seen;
                seen.insert((x, y, max_seen));
            }
            queue.push(right);
            num_seen += 1;
        }

        if up.2 >= 0
            && grid[y as usize][x as usize] == b'.'
            && grid[up.2 as usize][up.1 as usize] != b'#'
            && !seen.contains(&(up.1, up.2, index))
        {
            if num_seen > 0 {
                max_seen += 1;
                up.3 = max_seen;
                seen.insert((x, y, max_seen));
            }
            queue.push(up);
            num_seen += 1;
        }

        if down.2 < max_y
            && (grid[y as usize][x as usize] == b'.' || grid[y as usize][x as usize] == b'v')
            && grid[down.2 as usize][down.1 as usize] != b'#'
            && !seen.contains(&(down.1, down.2, index))
        {
            if num_seen > 0 {
                max_seen += 1;
                down.3 = max_seen;
                seen.insert((x, y, max_seen));
            }
            queue.push(down);
        }
        seen.insert((x, y, index));
    }
    answer
}

fn second_puzzle(input: &String) {
    let grid = input.lines().map(|line| line.bytes().collect()).collect();
    let answer = search2(&grid);
    println!("Puzzle two soln: {answer}");
}

fn search2(grid: &Vec<Vec<u8>>) -> usize {
    let mut nodes_set = HashSet::new();
    let mut nodes = Vec::new();
    let mut grid_to_node_ind = HashMap::new();
    let mut answer = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'#' {
                continue;
            }
            let mut neighbors = 0;
            if x > 0 && grid[y][x - 1] != b'#' {
                neighbors += 1;
            }

            if x < grid[0].len() - 1 && grid[y][x + 1] != b'#' {
                neighbors += 1;
            }

            if y > 0 && grid[y - 1][x] != b'#' {
                neighbors += 1;
            }

            if y < grid.len() - 1 && grid[y + 1][x] != b'#' {
                neighbors += 1;
            }

            if neighbors > 2 {
                if nodes_set.insert((x, y)) {
                    nodes.push((x, y));
                    grid_to_node_ind.insert((x, y), nodes.len() - 1);
                }
            }
        }
    }

    nodes.push((1, 0));
    nodes_set.insert((1, 0));
    let start_node_ind = nodes.len() - 1;
    grid_to_node_ind.insert((1, 0), start_node_ind);

    nodes.push((grid[0].len() - 2, grid.len() - 1));
    nodes_set.insert((grid[0].len() - 2, grid.len() - 1));
    let end_node_ind = nodes.len() - 1;
    grid_to_node_ind.insert((grid[0].len() - 2, grid.len() - 1), end_node_ind);

    let mut graph = HashMap::new();
    let mut visited = HashSet::new();

    for prev_node_ind in 0..nodes.len() {
        let (x, y) = nodes[prev_node_ind];
        let mut queue = VecDeque::new();
        queue.push_back((x, y, 0));

        while let Some((x, y, dist)) = queue.pop_front() {
            if nodes_set.contains(&(x, y)) {
                let node_ind = grid_to_node_ind[&(x, y)];
                if node_ind != prev_node_ind {
                    graph
                        .entry(prev_node_ind)
                        .or_insert_with(Vec::new)
                        .push((node_ind, dist));
                    graph
                        .entry(node_ind)
                        .or_insert_with(Vec::new)
                        .push((prev_node_ind, dist));
                    continue;
                }
            }

            if !visited.insert((x, y)) {
                continue;
            }

            if x > 0 && grid[y][x - 1] != b'#' {
                queue.push_back((x - 1, y, dist + 1));
            }

            if x < grid[0].len() - 1 && grid[y][x + 1] != b'#' {
                queue.push_back((x + 1, y, dist + 1));
            }

            if y > 0 && grid[y - 1][x] != b'#' {
                queue.push_back((x, y - 1, dist + 1));
            }

            if y < grid.len() - 1 && grid[y + 1][x] != b'#' {
                queue.push_back((x, y + 1, dist + 1));
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((start_node_ind, HashSet::new(), 0));
    while let Some((node_ind, visited, path_dist)) = queue.pop_front() {
        if node_ind == end_node_ind {
            answer = answer.max(path_dist);
        }
        let mut visited = visited.clone();
        if visited.insert(node_ind) {
            for &(next_node_ind, dist) in &graph[&node_ind] {
                queue.push_back((next_node_ind, visited.clone(), path_dist + dist));
            }
        }
    }
    answer
}
