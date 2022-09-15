const X: i32 = 1;
const Y: i32 = 2;
const Z: i32 = 3;

fn rot(start: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let (a, b, c) = start;
    vec![(a, b, c), (a, -b, -c), (a, -c, b), (a, c, -b)]
}

fn get_transforms() -> Vec<(i32, i32, i32)> {
    let mut result = vec![];
    for i in [-1, 1] {
        for a in [1, 2, 3] {
            let mut inner = match a {
                1 => rot((i * a, 2, 3)),
                2 => rot((i * a, 1, 3)),
                3 => rot((i * a, 1, 2)),
                _ => unreachable!(),
            };
            result.append(&mut inner);
        }
    }
    result
}

fn get_input() -> Vec<Vec<(i32, i32, i32)>> {
    todo!()
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
}
