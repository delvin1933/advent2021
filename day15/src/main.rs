use std::collections::HashMap;
use utils::read_lines;

extern crate pathfinding;

use pathfinding::prelude::dijkstra;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

fn parse_input(lines: Vec<String>) -> (HashMap<Point, u16>, usize, usize) {
    let mut result: HashMap<Point, u16> = HashMap::new();

    let mut max_x: usize = 0;

    for (y, line) in lines.iter().enumerate() {
        if y == 0 {
            max_x = line.len();
        }

        for (x, weight) in line.chars().enumerate() {
            result.insert(Point::new(x, y), weight.to_string().parse::<u16>().unwrap());
        }
    }
    (result, max_x - 1, lines.len() - 1)
}

fn compute_part1(input: &HashMap<Point, u16>, xmax: usize, ymax: usize) -> u16 {
    let goal = Point::new(xmax, ymax);

    match dijkstra(&Point::new(0, 0), |p| neighbours(p, &input), |p| *p == goal) {
        Some(result) => {
            //println!("Points : {:?}", result);
            result
                .0
                .iter()
                .fold(0, |sum, point| sum + input.get(point).unwrap())
                - 1
        }
        None => {
            println!("FAIL ");
            0
        }
    }
}

fn neighbours(p: &Point, points: &HashMap<Point, u16>) -> Vec<(Point, u16)> {
    let mut results: Vec<(Point, u16)> = Vec::new();

    if p.x > 0 {
        match points.get(&Point::new(p.x - 1, p.y)) {
            Some(weight) => {
                results.push((Point::new(p.x - 1, p.y), *weight));
            }
            _ => (),
        }
    }

    match points.get(&Point::new(p.x + 1, p.y)) {
        Some(weight) => {
            results.push((Point::new(p.x + 1, p.y), *weight));
        }
        _ => (),
    }

    match points.get(&Point::new(p.x, p.y + 1)) {
        Some(weight) => {
            results.push((Point::new(p.x, p.y + 1), *weight));
        }
        _ => (),
    }

    if p.y > 0 {
        match points.get(&Point::new(p.x, p.y - 1)) {
            Some(weight) => {
                results.push((Point::new(p.x, p.y - 1), *weight));
            }
            _ => (),
        }
    }

    results
}

fn expand_map(input: &HashMap<Point, u16>, max: usize) -> HashMap<Point, u16> {
    let mut result: HashMap<Point, u16> = HashMap::new();

    for add_y in 0..5 {
        for add_x in 0..5 {
            for (point, weight) in input.iter() {
                let mut new_weight = *weight + add_x + add_y;

                if new_weight > 9 {
                    new_weight = new_weight - 9;
                }

                let new_point = Point::new(
                    point.x + (max * add_x as usize),
                    point.y + (max * add_y as usize),
                );

                result.insert(new_point, new_weight);
            }
        }
    }
    result
}

// for debugging ...
fn print_map(input: &HashMap<Point, u16>, max: usize) {
    for y in 0..max {
        for x in 0..max {
            let p = input.get(&Point::new(x, y)).unwrap();
            print!("{}", p);
        }
        println!();
    }
}

fn main() {
    let lines = read_lines("./input.txt").unwrap_or_default();
    let (input, xmax, ymax) = parse_input(lines);

    let result = compute_part1(&input, xmax, ymax);

    println!("Part 1 : {}", result);

    let new_map = expand_map(&input, xmax + 1);

    let result = compute_part1(&new_map, 499, 499);
    println!("Part 2 : {}", result);
}

mod tests {
    use super::*;

    #[test]
    fn test_point_equal() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(1, 1);
        let p3 = Point::new(1, 2);
        assert_eq!(p1, p2);

        assert_ne!(p1, p3);
    }

    #[test]
    fn test_parse() {
        let lines: Vec<String> = vec![
            "1163751742".to_string(),
            "1381373672".to_string(),
            "2136511328".to_string(),
            "3694931569".to_string(),
            "7463417111".to_string(),
            "1319128137".to_string(),
            "1359912421".to_string(),
            "3125421639".to_string(),
            "1293138521".to_string(),
            "2311944581".to_string(),
        ];
        let (input, xmax, ymax) = parse_input(lines);

        let result = compute_part1(&input, xmax, ymax);
        assert_eq!(40, result);
    }

    #[test]
    fn test_parse2() {
        let lines: Vec<String> = vec![
            "1163751742".to_string(),
            "1381373672".to_string(),
            "2136511328".to_string(),
            "3694931569".to_string(),
            "7463417111".to_string(),
            "1319128137".to_string(),
            "1359912421".to_string(),
            "3125421639".to_string(),
            "1293138521".to_string(),
            "2311944581".to_string(),
        ];
        let (input, xmax, ymax) = parse_input(lines);

        let new_map = expand_map(&input, xmax + 1);

        let result = compute_part1(&new_map, 49, 49);
        assert_eq!(315, result);
    }
}
