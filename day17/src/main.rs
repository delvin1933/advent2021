use std::ops::Range;

fn step(x: i32, y: i32, vel_x: i32, vel_y: i32) -> (i32, i32, i32, i32) {
    let new_x = x + vel_x;
    let new_y = y + vel_y;

    let new_vel_x = if vel_x < 0 {
        0
    } else if vel_x > 0 {
        vel_x - 1
    } else {
        vel_x
    };

    let new_vel_y = vel_y - 1;

    (new_x, new_y, new_vel_x, new_vel_y)
}

fn is_within_target(x: i32, y: i32, range_x: Range<i32>, range_y: Range<i32>) -> bool {
    range_x.contains(&x) && range_y.contains(&y)
}

fn is_further(x: i32, y: i32, max_x: i32, max_y: i32) -> bool {
    x > max_x || y < max_y
}

fn shoot_probe(
    start_vel_x: i32,
    start_vel_y: i32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> Option<(i32, i32, i32)> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut apex_y: i32 = 0;

    let mut vel_x = start_vel_x;
    let mut vel_y = start_vel_y;

    while !is_further(x, y, max_x, min_y) {
        let (new_x, new_y, new_vel_x, new_vel_y) = step(x, y, vel_x, vel_y);
        if new_y > apex_y {
            apex_y = new_y
        }

        if is_within_target(new_x, new_y, min_x..max_x + 1, min_y..(max_y + 1)) {
            return Some((apex_y, start_vel_x, start_vel_y));
        }
        x = new_x;
        y = new_y;
        vel_x = new_vel_x;
        vel_y = new_vel_y;
    }

    None
}

fn part1(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> i32 {
    let mut apex: i32 = 0;

    let mut start_ok = 0;

    for y in (-max_x)..(max_x + 1) * 2 {
        for x in 0..max_x + 1 {
            match shoot_probe(x, y, min_x, max_x, min_y, max_y) {
                Some((new_apex, start_x, start_y)) => {
                    if new_apex > apex {
                        apex = new_apex
                    }
                    start_ok += 1;
                }
                None => (),
            }
        }
    }

    println!("Part 1 : {}", apex);
    println!("Part 2 : {}", start_ok);
    start_ok
}

fn main() {
    // "target area: x=235..259, y=-118..-62"

    part1(235, 259, -118, -62);
}

mod tests {

    use super::*;

    #[test]
    fn test_exemple1() {
        let (min_x, max_x, min_y, max_y) = (20, 30, -10, -5);

        let res = match shoot_probe(6, 9, min_x, max_x, min_y, max_y) {
            Some((res, _, _)) => res,
            None => 0,
        };

        assert_eq!(45, res);
    }

    #[test]
    fn test_exemple2() {
        let (min_x, max_x, min_y, max_y) = (20, 30, -10, -5);

        let res = match shoot_probe(9, 0, min_x, max_x, min_y, max_y) {
            Some((res, _, _)) => res,
            None => 1,
        };

        assert_eq!(0, res);
    }

    #[test]
    fn test_not_working() {
        let (min_x, max_x, min_y, max_y) = (20, 30, -10, -5);

        assert_eq!(None, shoot_probe(17, -4, min_x, max_x, min_y, max_y))
    }

    #[test]
    fn test_part2_example() {
        let res = part1(20, 30, -10, -5);
        assert_eq!(112, res);
    }
    #[test]
    fn test_is_further() {
        assert_eq!(false, is_further(20, -10, 30, -10))
    }
}
