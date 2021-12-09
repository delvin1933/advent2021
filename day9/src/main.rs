use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect())
}

fn main() {
    let lines = read_lines("./input.txt").unwrap_or_default();
    let map = parse_input(lines);

    let lowers = find_lowers(&map);

    println!("Part 1 : {}", calc_risk(&lowers, &map));

    let mut part2_points: Vec<HashSet<Point>> = Vec::new();
    for point in lowers {
        part2_points.push(part2_neighbours(point, &map));
        //lowers.iter().map(|l| ).collect();
    }

    let mut part2_counts: Vec<usize> = part2_points.iter().map(|s| s.len()).collect();

    part2_counts.sort();

    let count = part2_counts[part2_counts.len() - 1]
        * part2_counts[part2_counts.len() - 2]
        * part2_counts[part2_counts.len() - 3];

    println!("Part 2 : {}", count);
}

fn parse_input(input: Vec<String>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in input {
        let mut numline: Vec<i32> = Vec::new();
        for num in line.chars() {
            numline.push(num.to_string().parse::<i32>().unwrap());
        }
        result.push(numline);
    }

    result
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn find_lowers(map: &Vec<Vec<i32>>) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();

    for (y, yline) in map.iter().enumerate() {
        for (x, _value) in yline.iter().enumerate() {
            let current = Point { x, y };
            if is_low_point(&current, &map) {
                result.push(current);
            }
        }
    }

    result
}

fn is_low_point(point: &Point, map: &Vec<Vec<i32>>) -> bool {
    let max_x = map[0].len();
    let max_y = map.len();

    let x = point.x;
    let y = point.y;

    let upper_num: i32 = if y > 0 {
        match map.get(point.y - 1) {
            Some(line) => match line.get(x) {
                Some(value) => *value,
                None => 10,
            },
            None => 10,
        }
    } else {
        10
    };

    let left_num: i32 = if x > 0 {
        match map[y].get(x - 1) {
            Some(value) => *value,
            None => 10,
        }
    } else {
        10
    };

    let right_num: i32 = if x < max_x {
        match map[y].get(x + 1) {
            Some(value) => *value,
            None => 10,
        }
    } else {
        10
    };

    let lower_num: i32 = if y < max_y {
        match map.get(point.y + 1) {
            Some(line) => match line.get(x) {
                Some(value) => *value,
                None => 10,
            },
            None => 10,
        }
    } else {
        10
    };

    return map[point.y][point.x] < upper_num
        && map[point.y][point.x] < left_num
        && map[point.y][point.x] < right_num
        && map[point.y][point.x] < lower_num;
}

fn calc_risk(points: &Vec<Point>, map: &Vec<Vec<i32>>) -> i32 {
    points.iter().map(|p| map[p.y][p.x] + 1).sum()
}

fn part2_neighbours(point: Point, map: &Vec<Vec<i32>>) -> HashSet<Point> {
    let mut point_set: HashSet<Point> = HashSet::new();
    get_neighbours_inner(&mut point_set, &point, map);
    point_set
}

fn get_neighbours_inner(input_set: &mut HashSet<Point>, point: &Point, map: &Vec<Vec<i32>>) {
    if !input_set.contains(point) {
        if map[point.y][point.x] != 9 {
            if !input_set.contains(point) {
                input_set.insert(*point);
            }

            if point.y > 0 {
                get_neighbours_inner(
                    input_set,
                    &Point {
                        x: point.x,
                        y: point.y - 1,
                    },
                    map,
                );
            }
            if point.y < map.len() - 1 {
                get_neighbours_inner(
                    input_set,
                    &Point {
                        x: point.x,
                        y: point.y + 1,
                    },
                    map,
                );
            }

            if point.x > 0 {
                get_neighbours_inner(
                    input_set,
                    &Point {
                        x: point.x - 1,
                        y: point.y,
                    },
                    map,
                );
            }

            if point.x < map[0].len() - 1 {
                get_neighbours_inner(
                    input_set,
                    &Point {
                        x: point.x + 1,
                        y: point.y,
                    },
                    map,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower() {
        let lines = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];

        let map = parse_input(lines);
        println!("{:?}", map);
        let lowers = find_lowers(&map);
        assert_eq!(15, calc_risk(&lowers, &map));

        let mut part2_points: Vec<HashSet<Point>> = Vec::new();
        for point in lowers {
            part2_points.push(part2_neighbours(point, &map));
            //lowers.iter().map(|l| ).collect();
        }

        let mut part2_counts: Vec<usize> = part2_points.iter().map(|s| s.len()).collect();

        part2_counts.sort();
        let mut count = 1;

        count = part2_counts[part2_counts.len() - 1]
            * part2_counts[part2_counts.len() - 2]
            * part2_counts[part2_counts.len() - 3];
        assert_eq!(1134, count)
    }
}
