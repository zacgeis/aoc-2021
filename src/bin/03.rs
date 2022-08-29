use std::fs;
use std::iter;

fn main() {
    part1();
    part2();
}

fn get_input() -> Vec<Vec<u8>> {
    let input = fs::read_to_string("inputs/03.txt").unwrap();
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    c => panic!("invalid char: {}", c),
                })
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn bin_to_int(vals: Vec<u8>) -> u32 {
    vals.iter().fold(0, |val, &i| (val << 1) | i as u32)
}

fn part1() {
    let input = get_input();
    let char_count = input[0].len();
    let mut acc: Vec<i32> = vec![0; char_count];
    for line in input {
        for (i, &v) in line.iter().enumerate() {
            acc[i] += if v == 0 { -1 } else { 1 };
        }
    }
    let gamma: u32 = bin_to_int(acc.iter().map(|i| if *i > 0 { 1 } else { 0 }).collect());
    let mask = bin_to_int(iter::repeat(1).take(char_count).collect());
    let epsilon: u32 = !gamma & mask;

    println!("part1: {}", gamma * epsilon);
}

enum FilterType {
    MostCommon,
    LeastCommon,
}

fn part2_filter(lines: Vec<Vec<u8>>, filter_type: FilterType, pos: usize) -> Vec<Vec<u8>> {
    let mut acc = 0;
    for line in &lines {
        acc += if line[pos] == 0 { -1 } else { 1 }
    }
    let acc = match filter_type {
        FilterType::MostCommon => { if acc >= 0 { 1 } else { 0 } }
        FilterType::LeastCommon => { if acc >= 0 { 0 } else { 1 } }
    };
    lines.into_iter().filter(|line| line[pos] == acc).collect()
}

fn part2() {
    let input = get_input();
    let char_count = input[0].len();

    let mut generator = input.clone();
    for i in 0..char_count {
        generator = part2_filter(generator, FilterType::MostCommon, i);
        if generator.len() == 1 { break; };
    }

    let mut scrubber = input.clone();
    for i in 0..char_count {
        scrubber = part2_filter(scrubber, FilterType::LeastCommon, i);
        if scrubber.len() == 1 { break; };
    }

    let generator = bin_to_int(generator.pop().unwrap());
    let scrubber = bin_to_int(scrubber.pop().unwrap());

    println!("part1: {}", generator * scrubber);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filter_logic() {
        let input = vec![
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 2],
            vec![0, 0, 0, 3],
            vec![0, 0, 0, 4],
        ];

        let result = part2_filter(input, FilterType::MostCommon, 0);
        assert_eq!(result[0], [1, 0, 0, 1]);
    }
}
