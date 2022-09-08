use std::collections::HashMap;
use std::fs;

fn get_input() -> (Vec<char>, HashMap<(char, char), char>) {
    let input = fs::read_to_string("inputs/14.txt").unwrap();
    let mut lines = input.lines();
    let template = lines.next().unwrap();
    let template: Vec<char> = template.chars().collect();
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    for line in lines.skip(1) {
        let mut parts = line.split(" -> ");
        let mut from = parts.next().unwrap().chars();
        let to = parts.next().unwrap().chars().next().unwrap();
        let from = (from.next().unwrap(), from.next().unwrap());
        rules.insert(from, to);
    }
    (template, rules)
}

fn main() {
    println!("part1: {:?}", solve(10));
    println!("part2: {:?}", solve(40));
}

fn solve(steps: u32) -> u64 {
    let (template, rules) = get_input();

    let mut table: HashMap<(char, char), u64> = HashMap::new();
    for pair in template.windows(2) {
        let pair = (pair[0], pair[1]);
        let entry = table.entry(pair).or_insert(0);
        *entry += 1;
    }

    for _ in 0..steps {
        let mut new_table = table.clone();
        for (pair, count) in table {
            if let Some(target) = rules.get(&pair) {
                let left = (pair.0, *target);
                let right = (*target, pair.1);
                let old_value = new_table.entry(pair).or_insert(0);
                *old_value -= count;
                let left_value = new_table.entry(left).or_insert(0);
                *left_value += count;
                let right_value = new_table.entry(right).or_insert(0);
                *right_value += count;
            }
        }
        table = new_table;
    }

    let mut groups: HashMap<char, u64> = HashMap::new();
    for ((c, _), count) in table {
        let entry = groups.entry(c).or_insert(0);
        *entry += count;
    }
    let last_entry = groups.entry(*template.last().unwrap()).or_insert(0);
    *last_entry += 1;

    let max = groups.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    let min = groups.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

    max.1 - min.1
}
