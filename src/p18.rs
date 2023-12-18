use std::{fs, i64, time::Instant};

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p18.txt");
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
    let mut current = (0, 0);
    let mut graph: Vec<(i64, i64)> = Vec::new();
    let mut perimeter = 0;
    input.lines().for_each(|line| {
        let mut iter = line.split_whitespace();
        let direction = iter.next().unwrap();
        let steps: i64 = iter.next().unwrap().parse().unwrap();
        perimeter += steps;
        let next = match direction {
            "R" => (current.0 + steps, current.1),
            "L" => (current.0 - steps, current.1),
            "U" => (current.0, current.1 - steps),
            _ => (current.0, current.1 + steps),
        };
        graph.push(current);
        current = next;
    });
    graph.push(current);

    println!("Puzzle 1 soln: {}", find_area(&graph, perimeter));
}

fn second_puzzle(input: &String) {
    let mut current = (0, 0);
    let mut graph: Vec<(i64, i64)> = Vec::new();
    let mut perimeter = 0;
    input.lines().for_each(|line| {
        let mut iter = line.split_whitespace();
        iter.next(); // previous direction
        iter.next(); // wrong step
        let color = iter.next().unwrap();
        let steps = i64::from_str_radix(&color[2..7], 16).unwrap();
        let direction = color.chars().nth(7).unwrap();
        perimeter += steps;
        let next = match direction {
            '0' => (current.0 + steps, current.1),
            '2' => (current.0 - steps, current.1),
            '3' => (current.0, current.1 - steps),
            _ => (current.0, current.1 + steps),
        };
        graph.push(current);
        current = next;
    });
    graph.push(current);

    println!("Puzzle 2 soln: {}", find_area(&graph, perimeter));
}

fn find_area(graph: &Vec<(i64, i64)>, perimeter: i64) -> i64 {
    let mut area = 0;
    for i in 0..graph.len() - 1 {
        area += (graph[i].1 * graph[i + 1].0) - (graph[i].0 * graph[i + 1].1);
    }
    area.abs() / 2 + 1 + perimeter / 2
}
