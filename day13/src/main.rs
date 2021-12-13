use utils::read_lines;

use simple_matrix::{self, Matrix};

type DotMatrix = Matrix<u8>;

fn parse_input(lines: Vec<String>) -> (DotMatrix, Vec<(String, usize)>) {
    let mut coordonates: Vec<(usize, usize)> = Vec::new();

    let mut instructions: Vec<(String, usize)> = Vec::new();

    let mut xmax: usize = 0;
    let mut ymax: usize = 0;

    for line in lines {
        if line.contains("fold") {
            //instruction
            let mut raw_instr = line.split(" ");
            raw_instr.next();
            raw_instr.next();
            let mut raw_instr = raw_instr.next().unwrap().split("=");

            let axis = raw_instr.next().unwrap().to_string();
            let value: usize = raw_instr
                .next()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();

            instructions.push((axis, value));
        } else if line.len() > 0 {
            //coordonnée
            let mut raw_coord = line.split(",");
            let x_value: usize = raw_coord
                .next()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();

            if x_value > xmax {
                xmax = x_value;
            }
            let y_value: usize = raw_coord
                .next()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();

            if y_value > ymax {
                ymax = y_value;
            }

            coordonates.push((x_value, y_value));
        }
    }

    let mut matrix: DotMatrix = Matrix::new(xmax + 1, ymax + 1);

    for coord in coordonates {
        matrix.set(coord.0, coord.1, 1);
    }

    (matrix, instructions)
}

fn fold_matrix(matrix: &DotMatrix, axis: String, value: usize) -> DotMatrix {
    let (max_x, max_y, start_x, start_y) = if axis == "x".to_string() {
        (value, matrix.cols(), value, 0)
    } else {
        (matrix.rows(), value, 0, value)
    };

    let mut output: DotMatrix = DotMatrix::new(max_x, max_y);
    /*
    ...#..#..#
    ....#.....
    ..........
    #.........
    ...#....#.
    ..........
    ..........
    ----------
    ..........
    ..........
    .#....#.##
    ....#.....
    ......#...
    #.........

    (0, 14) -> (0, 0)

    value = 7
    y = 14

    new y = value - (y - value)
    new y = 7 - (14 -7 ) = 7 - 7 = 0

    value = 7
    y = 10

    new y = value - (y - value)
    new y = 7 - (10 - 7) = 7 - 3 = 4

    */

    for y in 0..=max_y {
        for x in 0..=max_x {
            match matrix.get(x, y) {
                Some(1) => {
                    output.set(x, y, 1);
                }
                _ => (),
            }
        }
    }

    for y in start_y..=(matrix.cols()) {
        for x in start_x..=(matrix.rows()) {
            match matrix.get(x, y) {
                Some(1) => {
                    if axis == "y".to_string() {
                        let new_y = value - (y as i32 - value as i32).abs() as usize;
                        output.set(x, new_y, 1);
                    } else {
                        let new_x = value - (x as i32 - value as i32).abs() as usize;
                        output.set(new_x, y, 1);
                    }
                }
                _ => (),
            };
        }
    }

    output
}

fn calc_dots(matrix: &DotMatrix) -> i32 {
    let mut result = 0;

    for y in 0..=matrix.cols() {
        for x in 0..=matrix.rows() {
            match matrix.get(x, y) {
                Some(1) => result += 1,
                _ => (),
            }
        }
    }

    result
}

fn print_matrix(matrice: &DotMatrix) {
    for y in 0..=matrice.cols() {
        for x in 0..=matrice.rows() {
            match matrice.get(x, y) {
                Some(1) => print!("#"),
                _ => print!("."),
            }
        }
        println!();
    }
}

fn main() {
    let lines = read_lines("./input.txt").unwrap_or_default();

    let (matrix, instructions) = parse_input(lines);

    let (axis, value) = instructions.iter().next().unwrap();
    let mut result = fold_matrix(&matrix, axis.clone(), *value);
    let res = calc_dots(&result);
    println!("Part 1 : {}", res);

    for (axis, value) in instructions.iter() {
        result = fold_matrix(&result, axis.clone(), *value);
    }

    print_matrix(&result);
}

mod tests {

    use super::*;

    #[test]
    fn test_() {
        let lines: Vec<String> = vec![
            "6,10".to_string(),
            "0,14".to_string(),
            "9,10".to_string(),
            "0,3".to_string(),
            "10,4".to_string(),
            "4,11".to_string(),
            "6,0".to_string(),
            "6,12".to_string(),
            "4,1".to_string(),
            "0,13".to_string(),
            "10,12".to_string(),
            "3,4".to_string(),
            "3,0".to_string(),
            "8,4".to_string(),
            "1,10".to_string(),
            "2,14".to_string(),
            "8,10".to_string(),
            "9,0".to_string(),
            "".to_string(),
            "fold along y=7".to_string(),
            "fold along x=5".to_string(),
        ];

        let (matrix, instructions) = parse_input(lines);

        print_matrix(&matrix);

        println!("{:?}", instructions);
        assert_eq!(
            vec![("y".to_string(), 7 as usize), ("x".to_string(), 5 as usize)],
            instructions
        );

        let result = fold_matrix(&matrix, "y".to_string(), 7);

        print_matrix(&result);

        let res = calc_dots(&result);
        assert_eq!(17, res);
    }
}
