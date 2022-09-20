use std::collections::HashMap;

struct Die {
    index: u64,
    count: u64,
}

impl Die {
    fn new() -> Self {
        Self { index: 1, count: 0 }
    }

    fn roll(&mut self) -> u64 {
        let mut value = 0;
        for _ in 0..3 {
            value += self.index;
            self.index = (self.index % 100) + 1;
        }
        self.count += 3;
        value
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut p1 = 7;
    let mut p2 = 8;
    let mut die = Die::new();
    loop {
        p1 = (((p1 - 1) + die.roll()) % 10) + 1;
        p1_score += p1;
        if p1_score >= 1000 {
            break;
        }

        p2 = (((p2 - 1) + die.roll()) % 10) + 1;
        p2_score += p2;
        if p2_score >= 1000 {
            break;
        }
    }
    let score = if p1_score > p2_score {
        p2_score * die.count
    } else {
        p1_score * die.count
    };
    println!("part1: {}", score);
}

#[derive(Copy, Clone)]
struct PlayerState {
    space: u8,
    score: u8,
    count: u8,
    acc: u8,
}

impl PlayerState {
    fn new(pos: u8) -> Self {
        Self {
            space: pos,
            score: 0,
            count: 0,
            acc: 0,
        }
    }

    fn roll(&self, roll: u8) -> Self {
        Self {
            space: self.space,
            score: self.score,
            count: self.count + 1,
            acc: self.acc + roll,
        }
    }

    fn turn(&self) -> Self {
        let new_space = (((self.space - 1) + self.acc) % 10) + 1;
        let new_score = self.score + new_space;
        Self {
            space: new_space,
            score: new_score,
            count: 0,
            acc: 0,
        }
    }
}

// memoize space and score for both players on count == 3.
// return win stats.
// also memoize the turns?
fn play(wins: &mut (u64, u64), p1_turn: bool, p1: PlayerState, p2: PlayerState) {
    let winning_score = 11; // 21
    if p1.score >= winning_score {
        wins.0 += 1;
    } else if p2.score >= winning_score {
        wins.1 += 1;
    } else if p1.count == 3 {
        play(wins, false, p1.turn(), p2);
    } else if p2.count == 3 {
        play(wins, true, p1, p2.turn());
    } else if p1_turn {
        play(wins, p1_turn, p1.roll(1), p2);
        play(wins, p1_turn, p1.roll(2), p2);
        play(wins, p1_turn, p1.roll(3), p2);
    } else {
        play(wins, p1_turn, p1, p2.roll(1));
        play(wins, p1_turn, p1, p2.roll(2));
        play(wins, p1_turn, p1, p2.roll(3));
    }
}

fn part2() {
    let mut wins = (0, 0);
    let p1 = PlayerState::new(4);
    let p2 = PlayerState::new(8);
    play(&mut wins, true, p1, p2);
    println!("wins2: {:?}", wins);
}
