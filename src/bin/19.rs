use std::fs;

const X: i32 = 1;
const Y: i32 = 2;
const Z: i32 = 3;

fn rot(start: [i32; 3]) -> Vec<[i32; 3]> {
    let [a, b, c] = start;
    vec![[a, b, c], [a, -b, -c], [a, -c, b], [a, c, -b]]
}

fn get_transforms() -> Vec<[i32; 3]> {
    let mut result = vec![];
    for i in [-1, 1] {
        for a in [X, Y, Z] {
            let mut inner = match a {
                X => rot([i * a, Y, Z]),
                Y => rot([i * a, X, Z]),
                Z => rot([i * a, X, Y]),
                _ => unreachable!(),
            };
            result.append(&mut inner);
        }
    }
    result
}

fn apply_transform(pos: &[i32; 3], transform: &[i32; 3]) -> [i32; 3] {
    let mut result = [0, 0, 0];
    result[0] = pos[(transform[0].abs() - 1) as usize];
    result[1] = pos[(transform[1].abs() - 1) as usize];
    result[2] = pos[(transform[2].abs() - 1) as usize];
    if transform[0] < 0 {
        result[0] *= -1;
    }
    if transform[1] < 0 {
        result[1] *= -1;
    }
    if transform[2] < 0 {
        result[2] *= -1;
    }
    result
}

fn get_input() -> Vec<Vec<[i32; 3]>> {
    let input = fs::read_to_string("inputs/19.txt").unwrap();
    let mut beacons = vec![];
    let mut scanners = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        } else if line.starts_with("---") {
            if !beacons.is_empty() {
                scanners.push(beacons);
            }
            beacons = vec![];
        } else {
            let values: Vec<i32> = line
                .split(',')
                .map(|value| value.parse().unwrap())
                .collect();
            beacons.push([values[0], values[1], values[2]]);
        }
    }
    scanners
}

fn main() {
    part1();
}

fn part1() {
    let transforms = get_transforms();
    println!("{}", transforms.len());
    // 1. start with scanner 0. transform all of the beacon locations and store them.
    // 2. iterate over beacons from the next scanner.
    //    for each beacon, diff it against one of the beacons in the 0 list.
    //    apply that diff to all other beacons in the list and see if we get 12 matches.
    //    if not, diff against the next entry.
    //    should only be O(27 * 27 * 27)
    // Scanner 0 is (0, 0, 0) so the offsets are easy.
    let scanners = get_input();
    println!("{:?}", &scanners[0]);
}

#[test]
fn test_apply_transform() {
    let start = [4, 5, 6];

    assert_eq!(apply_transform(&start, &[1, 2, 3]), [4, 5, 6]);
    assert_eq!(apply_transform(&start, &[3, 2, 1]), [6, 5, 4]);
    assert_eq!(apply_transform(&start, &[3, -2, -1]), [6, -5, -4]);
}
