use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(Eq, PartialEq)]
enum Space {
    Open,
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Space::Open,
            'A' => Space::Amber,
            'B' => Space::Bronze,
            'C' => Space::Copper,
            'D' => Space::Desert,
            _ => panic!("unexpected char found"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Space::Open => '.',
            Space::Amber => 'A',
            Space::Bronze => 'B',
            Space::Copper => 'C',
            Space::Desert => 'D',
        }
    }
}

type Move = ((u8, u8), (u8, u8));
struct Room {
    data: HashMap<(u8, u8), Space>,
}

impl fmt::Debug for Room {
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

impl Room {
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
        Room { data }
    }

    fn possible_moves(&self) -> Vec<Move> {
        todo!()
    }

    fn apply_move(&self, m: &Move) -> Self {
        todo!()
    }

    fn is_complete(&self) -> bool {
        for (i, c) in [Space::Amber, Space::Bronze, Space::Copper, Space::Desert].iter().enumerate() {
            let top = self.data.get(&(2 + i as u8 * 2, 1)).unwrap();
            let bot = self.data.get(&(2 + i as u8 * 2, 2)).unwrap();
            if top != c || bot != c {
                return false;
            }
        }
        true
    }
}

fn main() {
    let initial_positions = vec!['B', 'C', 'B', 'D', 'A', 'D', 'C', 'A'];
    // let initial_positions = vec!['A', 'B', 'C', 'D', 'A', 'B', 'C', 'D'];
    let room = Room::new(initial_positions);
    println!("{:?}", &room);
    println!("complete: {}", room.is_complete());
}
