use std::collections::HashSet;
use std::fs;

fn main() {
    part1();
    part2();
}

fn make_set(s: &str) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    for c in s.chars() {
        set.insert(c);
    }
    set
}

fn get_input() -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    let input = fs::read_to_string("inputs/08.txt").unwrap();
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" | ");
            let patterns = parts.next().unwrap();
            let values = parts.next().unwrap();

            let patterns = patterns
                .split_whitespace()
                .map(|word| make_set(&word))
                .collect::<Vec<HashSet<char>>>();
            let values = values
                .split_whitespace()
                .map(|word| make_set(&word))
                .collect::<Vec<HashSet<char>>>();

            (patterns, values)
        })
        .collect()
}

fn part1() {
    let input = get_input();

    let mut total = 0;
    for (_, values) in input {
        for value in values {
            match value.len() {
                2 => total += 1,
                3 => total += 1,
                4 => total += 1,
                7 => total += 1,
                _ => continue,
            }
        }
    }
    println!("part1: {}", total);
}

fn part2() {
    let input = get_input();

    let mut total = 0;
    for (patterns, values) in input {
        let one = patterns.iter().find(|pattern| pattern.len() == 2).unwrap();
        let seven = patterns.iter().find(|pattern| pattern.len() == 3).unwrap();
        let four = patterns.iter().find(|pattern| pattern.len() == 4).unwrap();
        let eight = patterns.iter().find(|pattern| pattern.len() == 7).unwrap();

        let top = seven - one;
        let partial_six = eight - seven;
        let partial_six = &partial_six | &top;
        let six = patterns
            .iter()
            .find(|pattern| pattern.len() == 6 && partial_six.is_subset(pattern))
            .unwrap();

        let partial_nine = four | &top;
        let nine = patterns
            .iter()
            .find(|pattern| pattern.len() == 6 && partial_nine.is_subset(pattern))
            .unwrap();

        let zero = patterns
            .iter()
            .find(|pattern| pattern.len() == 6 && pattern != &six && pattern != &nine)
            .unwrap();

        let three = patterns
            .iter()
            .find(|pattern| pattern.len() == 5 && seven.is_subset(pattern))
            .unwrap();

        let five = patterns
            .iter()
            .find(|pattern| pattern.len() == 5 && pattern.is_subset(six))
            .unwrap();

        let two = patterns
            .iter()
            .find(|pattern| pattern.len() == 5 && pattern != &three && pattern != &five)
            .unwrap();

        let digits = [zero, one, two, three, four, five, six, seven, eight, nine];

        let mut result = 0;
        for value in values {
            let pos = digits.iter().cloned().position(|pattern| pattern == &value);
            match pos {
                None => panic!("couldn't find pattern"),
                Some(i) => {
                    result *= 10;
                    result += i;
                }
            }
        }
        total += result;
    }

    println!("part2: {}", total);
}
