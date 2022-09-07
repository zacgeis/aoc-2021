use std::collections::{HashMap, HashSet};
use std::fs;

type Graph = HashMap<String, HashSet<String>>;

fn get_input() -> Graph {
    let input = fs::read_to_string("inputs/12.txt").unwrap();
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split('-');
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        let from_entry = graph.entry(from.to_string()).or_default();
        from_entry.insert(to.to_string());
        let to_entry = graph.entry(to.to_string()).or_default();
        to_entry.insert(from.to_string());
    }
    graph
}

fn main() {
    part1();
    part2();
}

fn find_paths<'a>(
    graph: &'a Graph,
    current: &'a str,
    mut path: Vec<&'a str>,
    mut visited: HashSet<&'a str>,
    allow_double: bool,
) -> Vec<Vec<&'a str>> {
    if current == "end" {
        path.push(current);
        vec![path]
    } else if visited.contains(current) {
        if allow_double && current != "start" {
            let mut new_visited = visited.clone();
            new_visited.remove(current);
            find_paths(graph, current, path.clone(), new_visited, false)
        } else {
            vec![]
        }
    } else {
        path.push(current);
        if current.chars().all(char::is_lowercase) {
            visited.insert(current);
        }
        let connections = &graph[&current.to_string()];
        let mut new_paths = vec![];
        for connection in connections {
            let new_path = path.clone();
            let new_visited = visited.clone();
            for new_path in find_paths(graph, connection, new_path, new_visited, allow_double) {
                new_paths.push(new_path);
            }
        }
        new_paths
    }
}

fn part1() {
    let graph = get_input();
    let start = match graph.keys().find(|key| *key == "start") {
        Some(end) => end,
        None => panic!("start not found?"),
    };
    let paths = find_paths(&graph, start, Default::default(), Default::default(), false);
    println!("path1: {}", paths.len());
}

fn part2() {
    let graph = get_input();
    let start = match graph.keys().find(|key| *key == "start") {
        Some(end) => end,
        None => panic!("start not found?"),
    };
    let paths = find_paths(&graph, start, Default::default(), Default::default(), true);
    println!("path2: {}", paths.len());
}
