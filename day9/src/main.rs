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

    let lowers = find_lowers(map);

    println!("Part 1 : {}", calc_risk(lowers));
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
struct Point {
    x: usize,
    y: usize,
    value: i32,
}

fn find_lowers(map: Vec<Vec<i32>>) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();

    for (y, yline) in map.iter().enumerate() {
        for (x, value) in yline.iter().enumerate() {
            let current = Point {
                x,
                y,
                value: map[y][x],
            };
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

fn calc_risk(points: Vec<Point>) -> i32 {
    points.iter().map(|p| p.value + 1).sum()
}

fn reduce_map(map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for yline in map {
        let mut line: Vec<i32> = Vec::new();
        for value in yline {
            if value == 9 {
                line.push(1);
            } else {
                line.push(0);
            }
        }
        result.push(line);
    }

    result
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
        let lowers = find_lowers(map);
        assert_eq!(15, calc_risk(lowers))
    }
}
