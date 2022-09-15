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
}
