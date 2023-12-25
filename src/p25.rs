use std::{
    collections::{HashMap, VecDeque},
    fs,
    time::Instant,
};

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p25.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return,
    };
    let now = Instant::now();
    first_puzzle(&input);
    let elapsed = Instant::elapsed(&now);
    println!("{:?}", elapsed);
}

fn first_puzzle(input: &String) {
    let mut graph = HashMap::new();
    let mut nodes = Vec::new();
    input.lines().for_each(|line| {
        let it = line.split_once(": ").unwrap();
        let key = it.0;
        let vals: Vec<&str> = it.1.split_whitespace().collect();
        if !graph.contains_key(key) {
            graph.insert(key.to_string(), HashMap::new());
            nodes.push(key.to_string());
        }
        for v in vals {
            graph.get_mut(key).unwrap().insert(v.to_string(), 1);
            if !graph.contains_key(v) {
                graph.insert(v.to_string(), HashMap::new());
                nodes.push(v.to_string());
            }
            graph.get_mut(v).unwrap().insert(key.to_string(), 1);
        }
    });

    let source = &nodes[0];
    for target in &nodes[1..] {
        let (cut, ans) = min_cut(&mut graph, source, target);
        if cut == 3 {
            println!("Puzzle soln: {ans}");
            break;
        }
    }
}

fn min_cut(
    graph: &mut HashMap<String, HashMap<String, isize>>,
    source: &String,
    target: &String,
) -> (isize, usize) {
    for (_, v) in graph.iter_mut() {
        for (_, c) in v.iter_mut() {
            *c = 1;
        }
    }
    let mut max_flow = 0;
    let mut parent: HashMap<String, String>;
    loop {
        parent = bfs(graph.clone(), source);
        if !parent.contains_key(target) {
            break;
        }
        let mut flow = isize::MAX;
        let mut n = target;
        while n != source {
            flow = flow.min(graph[&parent[n]][n]);
            n = &parent[n];
        }
        max_flow += flow;

        let mut v = target;
        while v != source {
            let u = &parent[v];
            let c = graph.get_mut(u).unwrap().get_mut(v).unwrap();
            *c -= flow;
            let c = graph.get_mut(v).unwrap().get_mut(u).unwrap();
            *c += flow;
            v = u;
        }
    }
    let mut ans = 0;
    if max_flow == 3 {
        let g = parent.len();
        ans = (graph.len() - g) * g;
    }

    (max_flow, ans)
}

fn bfs(graph: HashMap<String, HashMap<String, isize>>, source: &String) -> HashMap<String, String> {
    let mut parent = HashMap::new();
    parent.insert(source.to_string(), source.to_string());
    let mut queue = VecDeque::new();
    queue.push_back(source);
    while let Some(node) = queue.pop_front() {
        for (e, c) in graph[node].iter() {
            if *c > 0 && !parent.contains_key(e) {
                parent.insert(e.to_string(), node.to_string());
                queue.push_back(e);
            }
        }
    }
    parent
}
