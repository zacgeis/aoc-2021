use std::cmp::max;

fn sim(vel: (i32, i32)) -> Option<i32> {
    let top_left = (244, -54);
    let bottom_right = (303, -91);

    let (mut v_x, mut v_y) = vel;
    let mut p_x = 0;
    let mut p_y = 0;
    let mut h_y = i32::MIN;
    loop {
        p_x += v_x;
        p_y += v_y;
        h_y = max(p_y, h_y);
        if v_x > 0 {
            v_x -= 1;
        } else if v_x < 0 {
            v_x += 1;
        }
        v_y -= 1;

        if p_y < bottom_right.1 && v_y < 0 {
            break None;
        }
        if p_x >= top_left.0 && p_x <= bottom_right.0 && p_y <= top_left.1 && p_y >= bottom_right.1
        {
            break Some(h_y);
        }
    }
}

fn main() {
    let mut t = 0;
    let mut h = 0;
    for v_x in -500..500 {
        for v_y in -500..500 {
            if let Some(r) = sim((v_x, v_y)) {
                t += 1;
                h = max(h, r);
            }
        }
    }
    println!("part1: {}", h);
    println!("part2: {}", t);
}
