use std::cmp::Ordering;
use std::collections::{HashSet, BinaryHeap, HashMap};

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

    fn cost(&self) -> u64 {
        match self {
            Amphipods::Amber => 1,
            Amphipods::Bronze => 10,
            Amphipods::Copper => 100,
            Amphipods::Desert => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Node {
    Hallway(Option<Amphipods>),
    HallwayMoveOnly,
    Home(Amphipods, Option<Amphipods>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Graph {
    cost: u64,
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
}

impl Ord for Graph {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Graph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    fn new(positions: &[char]) -> Self {
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
            let node_1 = Node::Home(*a, None);
            let node_1_i = nodes.len();
            nodes.push(node_1);
            edges.push(vec![]);

            let node_2 = Node::Home(*a, None);
            let node_2_i = nodes.len();
            nodes.push(node_2);
            edges.push(vec![]);

            let node_3 = Node::Home(*a, None);
            let node_3_i = nodes.len();
            nodes.push(node_3);
            edges.push(vec![]);

            let node_4 = Node::Home(*a, None);
            let node_4_i = nodes.len();
            nodes.push(node_4);
            edges.push(vec![]);

            let hallway_i = 2 + (i * 2);
            edges[hallway_i].push(node_1_i);

            edges[node_1_i].push(hallway_i);
            edges[node_1_i].push(node_2_i);

            edges[node_2_i].push(node_1_i);
            edges[node_2_i].push(node_3_i);

            edges[node_3_i].push(node_2_i);
            edges[node_3_i].push(node_4_i);

            edges[node_4_i].push(node_3_i);
        }
        let mut graph = Graph {
            cost: 0,
            nodes,
            edges,
        };
        graph.set_positions(positions);
        graph
    }

    fn set_positions(&mut self, positions: &[char]) {
        let mut homes = vec![];
        homes.append(&mut self.get_homes(&Amphipods::Amber));
        homes.append(&mut self.get_homes(&Amphipods::Bronze));
        homes.append(&mut self.get_homes(&Amphipods::Copper));
        homes.append(&mut self.get_homes(&Amphipods::Desert));
        for (i, c) in positions.iter().enumerate() {
            self.place(homes[i], &Amphipods::from_char(*c));
        }
    }

    fn is_complete(&self) -> bool {
        for node in &self.nodes {
            if let Node::Home(a, b) = node {
                match b {
                    None => return false,
                    Some(b) => {
                        if a != b {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn get_homes(&self, a: &Amphipods) -> Vec<usize> {
        let mut homes = vec![];
        for (i, node) in self.nodes.iter().enumerate() {
            if let Node::Home(b, _) = node {
                if a == b {
                    homes.push(i);
                }
            }
        }
        homes
    }

    fn is_home_open(&self, home: usize) -> bool {
        match self.nodes[home] {
            Node::Home(_, None) => true,
            _ => false,
        }
    }

    fn are_homes_available(&self, a: &Amphipods) -> bool {
        for home in self.get_homes(a) {
            if !self.is_home_open(home) {
                let b = self.get_type(home).unwrap();
                if a != b {
                    return false;
                }
            }
        }
        true
    }

    fn get_type(&self, i: usize) -> Option<&Amphipods> {
        match &self.nodes[i] {
            Node::Hallway(Some(a)) => Some(a),
            Node::Home(_, Some(a)) => Some(a),
            _ => None,
        }
    }

    fn clear(&mut self, i: usize) {
        match &self.nodes[i] {
            Node::Hallway(Some(_)) => {
                self.nodes[i] = Node::Hallway(None);
            }
            Node::Home(t, Some(_)) => {
                self.nodes[i] = Node::Home(*t, None);
            }
            _ => panic!("can't clear"),
        }
    }

    fn place(&mut self, i: usize, a: &Amphipods) {
        match &self.nodes[i] {
            Node::Hallway(None) => {
                self.nodes[i] = Node::Hallway(Some(*a));
            }
            Node::Home(t, None) => {
                self.nodes[i] = Node::Home(*t, Some(*a));
            }
            _ => panic!("can't place"),
        }
    }

    fn try_move(&self, from: usize, to: usize) -> Option<Self> {
        let from_type = self.get_type(from).unwrap();
        let mut visited = HashSet::new();
        let mut stack = vec![];
        stack.push((from, 0));
        while let Some((current, cost)) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            if current == to {
                let mut new_graph = self.clone();
                new_graph.clear(from);
                new_graph.place(to, from_type);
                new_graph.cost += cost;
                // println!("moved from {:?} to {:?}", &self.nodes[from], &self.nodes[to]);
                return Some(new_graph);
            }
            for i in &self.edges[current] {
                match &self.nodes[*i] {
                    Node::Hallway(None) | Node::HallwayMoveOnly | Node::Home(_, None) => {
                        stack.push((*i, cost + from_type.cost()));
                    }
                    _ => (),
                }
            }
        }
        None
    }

    fn get_open_hallways(&self) -> Vec<usize> {
        let mut result = vec![];
        for (i, node) in self.nodes.iter().enumerate() {
            if let Node::Hallway(None) = node {
                result.push(i);
            }
        }
        result
    }

    fn next_states(&self) -> Vec<Self> {
        let mut results = vec![];
        'node: for (i, node) in self.nodes.iter().enumerate() {
            match node {
                Node::Hallway(Some(a)) => {
                    if self.are_homes_available(a) {
                        for home in self.get_homes(a).iter().rev() {
                            if let Some(next) = self.try_move(i, *home) {
                                results.push(next);
                                continue 'node;
                            }
                        }
                    }
                }
                Node::Home(h, Some(a)) => {
                    if h != a {
                        if self.are_homes_available(a) {
                            for home in self.get_homes(a).iter().rev() {
                                if let Some(next) = self.try_move(i, *home) {
                                    results.push(next);
                                    continue 'node;
                                }
                            }
                        }
                        for j in self.get_open_hallways() {
                            if let Some(next) = self.try_move(i, j) {
                                results.push(next);
                            }
                        }
                    } else {
                        for home in &self.get_homes(a)[1..] {
                            if let Some(home_type) = self.get_type(*home) {
                                if home_type != a {
                                    for j in self.get_open_hallways() {
                                        if let Some(next) = self.try_move(i, j) {
                                            results.push(next);
                                        }
                                    }
                                    continue 'node;
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        results
    }
}

fn main() {
    let initial_positions = ['D', 'D', 'D', 'C',
                             'A', 'C', 'B', 'A',
                             'C', 'B', 'A', 'B',
                             'D', 'A', 'C', 'B'];
    // let initial_positions = ['B', 'D', 'D', 'A',
    //                          'C', 'C', 'B', 'D',
    //                          'B', 'B', 'A', 'C',
    //                          'D', 'A', 'C', 'A'];
    // let initial_positions = ['B', 'A', 'A', 'A',
    //                          'A', 'B', 'B', 'B',
    //                          'C', 'D', 'C', 'C',
    //                          'D', 'C', 'D', 'D'];
    let graph = Graph::new(&initial_positions);
    let mut heap = BinaryHeap::new();
    heap.push(graph);
    let mut lowest_cost = u64::MAX;
    let mut counter = 0;
    while let Some(graph) = heap.pop() {
        if graph.is_complete() {
            lowest_cost = graph.cost;
            break;
        }
        for next_graph in graph.next_states() {
            heap.push(next_graph);
        }
        counter += 1;
        if counter >= 10_000 {
            let mut compact = HashMap::new();
            for mut graph in heap.drain() {
                let cost = graph.cost;
                graph.cost = 0;
                let value = compact.entry(graph).or_insert(u64::MAX);
                if cost < *value {
                    *value = cost;
                }
            }
            for (mut graph, cost) in compact {
                graph.cost = cost;
                heap.push(graph);
            }
            counter = 0;
        }
    }
    println!("part2: {}", lowest_cost);
}
