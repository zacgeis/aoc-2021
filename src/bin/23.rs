use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Amphipods {
    Amber,
    Bronze,
    Copper,
    Desert,
}
#[derive(Hash, Eq, PartialEq)]
enum Space {
    Open,
    Used(Amphipods),
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

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Space::Open,
            c => Space::Used(Amphipods::from_char(c)),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Space::Open => '.',
            Space::Used(a) => a.to_char(),
        }
    }
}

type Pos = (u8, u8);
type Move = (Pos, u8); // and cost.
struct Map {
    data: HashMap<Pos, Space>,
    homes: HashMap<Amphipods, HashSet<Pos>>,
    move_only: HashSet<Pos>,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..3 {
            for x in 0..11 {
                let c = match self.data.get(&(x, y)) {
                    Some(space) => space.to_char(),
                    None => '#',
                };
                write!(f, "{}", c)?;
            }
            if y != 2 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl Map {
    fn new(mut initial_positions: Vec<char>) -> Self {
        initial_positions.reverse();
        let mut data = HashMap::new();
        for i in 0..11 {
            data.insert((i, 0), Space::Open);
        }
        for i in 0..4 {
            let x = 2 + i * 2;
            data.insert((x, 1), Space::from_char(initial_positions.pop().unwrap()));
        }
        for i in 0..4 {
            let x = 2 + i * 2;
            data.insert((x, 2), Space::from_char(initial_positions.pop().unwrap()));
        }
        let mut homes = HashMap::new();
        let mut move_only = HashSet::new();
        for (i, a) in Amphipods::VALUES.iter().enumerate() {
            let x = 2 + i * 2;
            move_only.insert((x as u8, 0 as u8));
            let mut section = HashSet::new();
            section.insert((x as u8, 1 as u8));
            section.insert((x as u8, 2 as u8));
            homes.insert(*a, section);
        }
        Map { data, homes, move_only }
    }

    fn possible_moves(&self, pos: Pos) -> Vec<Move> {
        let s = self.data.get(&pos).unwrap_or(&Space::Open);
        match s {
            Space::Open => vec![],
            Space::Used(a) => {
                // in home, but need to move to let bottom one out.
                //   move out.
                // in home and are the bottom.
                //   do nothing.
                // are in a different home.
                //   move out.
                // are in the hallway.
                //   only move into home.
                // can't block any of the rooms with any moves.
                // can't move into a room that contains one of the wrong types.
                if self.homes[a].contains(&pos) {
                    if pos.1 == 1 {
                        let other = &self.data[&(pos.0, 2)];
                        match other {
                            Space::Open => (),
                            Space::Used(other) => {
                                if a != other {
                                    // need to let the bottom out.
                                }
                            }
                        }
                    }
                } else {
                    if pos.1 > 0 {
                        // in another home.
                    } else {
                        // in the hallway. can only move into home.
                    }
                }
                todo!()
            }
        }
    }

    fn is_complete(&self) -> bool {
        for (i, c) in Amphipods::VALUES.iter().enumerate() {
            let top = self.data.get(&(2 + i as u8 * 2, 1)).unwrap();
            let bot = self.data.get(&(2 + i as u8 * 2, 2)).unwrap();
            match (top, bot) {
                (Space::Used(a), Space::Used(b)) => {
                    if a != c || b != c {
                        return false;
                    }
                }
                _ => (),
            }
        }
        true
    }
}

fn main() {
    let initial_positions = vec!['B', 'C', 'B', 'D', 'A', 'D', 'C', 'A'];
    // let initial_positions = vec!['A', 'B', 'C', 'D', 'A', 'B', 'C', 'D'];
    let map = Map::new(initial_positions);
    println!("{:?}", &map);
    println!("complete: {}", map.is_complete());
}
