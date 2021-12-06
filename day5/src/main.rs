use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
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

#[derive(Default, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn parse_from_string(input: String) -> Point {
        let mut split_input = input.split(',');

        Point {
            x: split_input.next().unwrap().parse::<i32>().unwrap(),
            y: split_input.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

#[derive(Default, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn parse_from_string(input: String) -> Line {
        let mut split_input = input.split(" -> ");
        let start = Point::parse_from_string(split_input.next().unwrap().to_string());
        let end = Point::parse_from_string(split_input.next().unwrap().to_string());

        Line { start, end }
    }

    fn is_along_axes(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn compute_points(&self) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();

        let mut x = self.start.x;
        let mut y = self.start.y;
        result.push(Point { x, y });

        while x != self.end.x || y != self.end.y {
            x -= (x > self.end.x) as i32;
            x += (x < self.end.x) as i32;
            y -= (y > self.end.y) as i32;
            y += (y < self.end.y) as i32;
            result.push(Point { x, y });
        }

        result
    }
}

fn main() {
    let lines = read_lines("./input.txt").unwrap_or_default();

    let lines: Vec<Line> = lines
        .iter()
        .map(|l| Line::parse_from_string(l.to_string()))
        .collect();

    let axis_lines = lines.iter().filter(|l| l.is_along_axes());

    let part1_points: Vec<Point> = axis_lines.flat_map(|l| l.compute_points()).collect();

    let mut result: HashMap<Point, i32> = HashMap::new();

    for item in part1_points {
        *result.entry(item).or_insert(0) += 1;
    }

    let count_occ = result
        .into_iter()
        .filter(|(_, v)| *v >= 2)
        .collect::<Vec<(Point, i32)>>();

    println!("Part 1 {:?}", count_occ.len());

    let part2_points: Vec<Point> = lines.iter().flat_map(|l| l.compute_points()).collect();

    let mut result: HashMap<Point, i32> = HashMap::new();

    for item in part2_points {
        *result.entry(item).or_insert(0) += 1;
    }

    let count_occ = result
        .into_iter()
        .filter(|(_, v)| *v >= 2)
        .collect::<Vec<(Point, i32)>>();

        println!("Part 2 {:?}", count_occ.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 1, y: 1 };
        assert_eq!(p1, p2);
    }
}
