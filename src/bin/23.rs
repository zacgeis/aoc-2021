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
        let mut graph = Graph {
            cost: 0,
            nodes,
            edges,
        };
        graph.set_positions(positions);
        graph
    }

    fn set_positions(&mut self, positions: &[char]) {
        assert!(positions.len() == 8);
        let (a1, a5) = self.get_homes(&Amphipods::Amber);
        let (a2, a6) = self.get_homes(&Amphipods::Bronze);
        let (a3, a7) = self.get_homes(&Amphipods::Copper);
        let (a4, a8) = self.get_homes(&Amphipods::Desert);
        let mapping = [a1, a2, a3, a4, a5, a6, a7, a8];
        for (i, c) in positions.iter().enumerate() {
            self.place(mapping[i], &Amphipods::from_char(*c));
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

    fn get_homes(&self, a: &Amphipods) -> (usize, usize) {
        let mut top = 0;
        let mut bot = 0;
        for (i, node) in self.nodes.iter().enumerate() {
            if let Node::Home(b, _) = node {
                if a == b {
                    if self.edges[i].len() == 1 {
                        bot = i;
                    } else {
                        top = i;
                    }
                }
            }
        }
        assert!(top != 0);
        assert!(bot != 0);
        (top, bot)
    }

    fn is_home_open(&self, home: usize) -> bool {
        match self.nodes[home] {
            Node::Home(_, None) => true,
            _ => false,
        }
    }

    fn are_homes_available(&self, a: &Amphipods) -> bool {
        let (top_home, bot_home) = self.get_homes(a);
        if !self.is_home_open(top_home) {
            let b = self.get_type(top_home).unwrap();
            if a != b {
                return false;
            }
        }
        if !self.is_home_open(bot_home) {
            let b = self.get_type(bot_home).unwrap();
            if a != b {
                return false;
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
        for (i, node) in self.nodes.iter().enumerate() {
            match node {
                Node::Hallway(Some(a)) => {
                    if self.are_homes_available(a) {
                        let (top_home, bot_home) = self.get_homes(a);
                        if let Some(next) = self.try_move(i, bot_home) {
                            results.push(next);
                            continue;
                        }
                        if let Some(next) = self.try_move(i, top_home) {
                            results.push(next);
                            continue;
                        }
                    }
                }
                Node::Home(h, Some(a)) => {
                    if h != a {
                        if self.are_homes_available(a) {
                            let (top_home, bot_home) = self.get_homes(a);
                            if let Some(next) = self.try_move(i, bot_home) {
                                results.push(next);
                                continue;
                            }
                            if let Some(next) = self.try_move(i, top_home) {
                                results.push(next);
                                continue;
                            }
                        }
                        for j in self.get_open_hallways() {
                            if let Some(next) = self.try_move(i, j) {
                                results.push(next);
                            }
                        }
                    } else {
                        let (_, bot_home) = self.get_homes(a);
                        if let Some(bot_type) = self.get_type(bot_home) {
                            if bot_type != a {
                                for j in self.get_open_hallways() {
                                    if let Some(next) = self.try_move(i, j) {
                                        results.push(next);
                                    }
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
    let initial_positions = ['D', 'A', 'C', 'D',
                             'C', 'A', 'B', 'B'];
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
        if counter >= 10000 {
            let mut compact = HashMap::new();
            for graph in heap.drain() {
                let cost = graph.cost;
                let value = compact.entry(graph).or_insert(u64::MAX);
                if cost < *value {
                    *value = cost;
                }
            }
            for (graph, _) in compact {
                heap.push(graph);
            }
            counter = 0;
        }
    }
    println!("lowest_cost: {:?}", lowest_cost);
}
