use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Amphipods {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipods {
    const VALUES: [Self; 4] = [Self::Amber, Self::Bronze, Self::Copper, Self::Desert];

    fn from_char(c: char) -> Self {
        match c {
            'A' => Amphipods::Amber,
            'B' => Amphipods::Bronze,
            'C' => Amphipods::Copper,
            'D' => Amphipods::Desert,
            _ => panic!("unexpected char found"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Amphipods::Amber => 'A',
            Amphipods::Bronze => 'B',
            Amphipods::Copper => 'C',
            Amphipods::Desert => 'D',
        }
    }
}

#[derive(Debug)]
enum Node {
    Hallway(Option<Amphipods>),
    HallwayMoveOnly,
    Home(Amphipods, Option<Amphipods>),
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        let mut nodes = vec![];
        let mut edges = vec![];
        let room_x = [2, 4, 6, 8];
        for i in 0..11 {
            let node = if room_x.contains(&i) {
                Node::HallwayMoveOnly
            } else {
                Node::Hallway(None)
            };
            nodes.push(node);
            let edge = if i == 0 {
                vec![1]
            } else if i == 10 {
                vec![9]
            } else {
                vec![i - 1, i + 1]
            };
            edges.push(edge);
        }
        for (i, a) in Amphipods::VALUES.iter().enumerate() {
            let node_top = Node::Home(*a, None);
            let node_top_i = nodes.len();
            nodes.push(node_top);

            let node_bot = Node::Home(*a, None);
            let node_bot_i = nodes.len();
            nodes.push(node_bot);

            let hallway_i = 2 + (i * 2);
            edges[hallway_i].push(node_top_i);
            edges.push(vec![hallway_i, node_bot_i]);
            edges.push(vec![node_top_i]);
        }
        Graph { nodes, edges }
    }

    fn get_homes(&self, a: &Amphipods) -> Vec<usize> {
        let mut results = vec![];
        for (i, node) in self.nodes.iter().enumerate() {
            if let Node::Home(t, _) =  node {
                if t == a {
                    results.push(i);
                }
            }
        }
        results
    }
}

fn main() {
    let initial_positions = vec!['B', 'C', 'B', 'D', 'A', 'D', 'C', 'A'];
    // let initial_positions = vec!['A', 'B', 'C', 'D', 'A', 'B', 'C', 'D'];
    let graph = Graph::new();
    println!("graph: {:?}", graph);
    // let map = Map::new(initial_positions);
    // println!("{:?}", &map);
    // println!("complete: {}", map.is_complete());
}
