use std::fs;

const X: i32 = 1;
const Y: i32 = 2;
const Z: i32 = 3;

fn rot_x(start: [i32; 3]) -> [i32; 3] {
    let [x, y, z] = start;
    [x, z, -y]
}

fn rot_y(start: [i32; 3]) -> [i32; 3] {
    let [x, y, z] = start;
    [z, y, -x]
}

fn get_transforms() -> Vec<[i32; 3]> {
    let origin = [X, Y, Z];
    let bases = [
        origin,
        rot_y(origin),
        rot_y(rot_y(origin)),
        rot_y(rot_y(rot_y(origin))),
        rot_y(rot_x(origin)),
        rot_y(rot_y(rot_y(rot_x(origin)))),
    ];

    let mut results = vec![];
    for base in bases {
        results.push(base);
        results.push(rot_x(base));
        results.push(rot_x(rot_x(base)));
        results.push(rot_x(rot_x(rot_x(base))));
    }

    results
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

fn dist(a: &[i32; 3]) -> i32 {
    a[0] + a[1] + a[2]
}

fn abs(a: &[i32; 3]) -> [i32; 3] {
    [a[0].abs(), a[1].abs(), a[2].abs()]
}

fn sub(a: &[i32; 3], b: &[i32; 3]) -> [i32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn add(a: &[i32; 3], b: &[i32; 3]) -> [i32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
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
    if !beacons.is_empty() {
        scanners.push(beacons);
    }
    scanners
}

fn find_offset(origin_beacons: &[[i32; 3]], beacons: &[[i32; 3]]) -> Option<([i32; 3], [i32; 3])> {
    let transforms = get_transforms();
    for transform in &transforms {
        for beacon in beacons {
            let transformed_beacon = apply_transform(beacon, transform);
            for origin_beacon in origin_beacons {
                let possible_origin = sub(origin_beacon, &transformed_beacon);
                let mut count = 0;
                for second_beacon in beacons {
                    let second_beacon =
                        add(&possible_origin, &apply_transform(second_beacon, transform));
                    if origin_beacons.contains(&second_beacon) {
                        count += 1;
                        if count == 12 {
                            return Some((possible_origin, *transform));
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let mut scanners = get_input().into_iter();
    let mut origin_beacons = scanners.next().unwrap();
    let mut scanners: Vec<_> = scanners.collect();
    let mut scanner_positions = vec![];
    while !scanners.is_empty() {
        let mut new_scanners: Vec<Vec<[i32; 3]>> = vec![];
        for beacons in scanners {
            match find_offset(&origin_beacons, &beacons) {
                Some((origin, transform)) => {
                    for beacon in &beacons {
                        scanner_positions.push(origin);
                        let transformed = add(&origin, &apply_transform(beacon, &transform));
                        if !origin_beacons.contains(&transformed) {
                            origin_beacons.push(transformed);
                        }
                    }
                }
                None => {
                    new_scanners.push(beacons);
                }
            }
        }
        scanners = new_scanners;
    }

    println!("part1: {}", origin_beacons.len());

    let mut p2 = 0;
    for a in &scanner_positions {
        for b in &scanner_positions {
            let d = dist(&abs(&sub(a, b)));
            if d > p2 {
                p2 = d;
            }
        }
    }

    println!("part2: {}", p2);
}

#[test]
fn test_apply_transform() {
    let start = [4, 5, 6];

    assert_eq!(apply_transform(&start, &[1, 2, 3]), [4, 5, 6]);
    assert_eq!(apply_transform(&start, &[3, 2, 1]), [6, 5, 4]);
    assert_eq!(apply_transform(&start, &[3, -2, -1]), [6, -5, -4]);
}
