use std::str::Matches;

use simple_matrix::Matrix;

type MatrixOctopus = Matrix<u32>;

fn step(matrice: &mut MatrixOctopus) -> i32 {
    for x in 0..matrice.rows() {
        for y in 0..matrice.cols() {
            incr_energy(x, y, matrice);
        }
    }
    let mut nb_changed = 0;
    for x in 0..matrice.rows() {
        for y in 0..matrice.cols() {
            let value = matrice.get(x, y).unwrap();
            if *value > 9 {
                nb_changed += 1;
                matrice.set(x, y, 0);
            }
        }
    }
    nb_changed
}

fn incr_energy(x: usize, y: usize, matrice: &mut MatrixOctopus) {
    match matrice.get(x, y) {
        Some(value) => {
            if *value < 9 {
                matrice.set(x, y, value + 1);
            } else if *value == 9 {
                matrice.set(x, y, 10);
                println!("{}, {}", x, y);

                if x > 0 && y > 0 {
                    incr_energy(x - 1, y - 1, matrice);
                }
                if x > 0 {
                    incr_energy(x - 1, y, matrice);
                    incr_energy(x - 1, y + 1, matrice);
                }
                if y > 0 {
                    incr_energy(x, y - 1, matrice);
                    incr_energy(x + 1, y - 1, matrice);
                }

                incr_energy(x + 1, y, matrice);

                incr_energy(x, y + 1, matrice);
                incr_energy(x + 1, y + 1, matrice);
            }
        }
        None => (),
    };
}

fn is_synchronised(matrice: &MatrixOctopus) -> bool {
    let value = matrice.get(0, 0).unwrap();
    for x in 0..matrice.rows() {
        for y in 0..matrice.cols() {
            if *value != *matrice.get(x, y).unwrap() {
                return false;
            }
        }
    }
    true
}

fn parse_input(input: Vec<String>, size: usize) -> MatrixOctopus {
    let mut m = MatrixOctopus::new(size, size);

    for (x, octopusline) in input.iter().enumerate() {
        for (y, value) in octopusline.chars().enumerate() {
            m.set(x, y, value.to_string().parse::<u32>().unwrap());
        }
    }
    m
}

fn main() {
    let mut matrice: MatrixOctopus = parse_input(
        vec![
            "2344671212".to_string(),
            "6611742681".to_string(),
            "5575575573".to_string(),
            "3167848536".to_string(),
            "1353827311".to_string(),
            "4416463266".to_string(),
            "2624761615".to_string(),
            "1786561263".to_string(),
            "3622643215".to_string(),
            "4143284653".to_string(),
        ],
        10,
    );
    let mut count = 0;

    for _i in 0..100 {
        count += step(&mut matrice.clone());
    }
    println!("Part 1 : {}", count);

    count = 0;
    while !is_synchronised(&matrice) {
        step(&mut matrice);
        count += 1;
    }
    println!("Part 2 : {}", count);
}

mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut matrice: MatrixOctopus = parse_input(
            vec![
                "5483143223".to_string(),
                "2745854711".to_string(),
                "5264556173".to_string(),
                "6141336146".to_string(),
                "6357385478".to_string(),
                "4167524645".to_string(),
                "2176841721".to_string(),
                "6882881134".to_string(),
                "4846848554".to_string(),
                "5283751526".to_string(),
            ],
            10,
        );

        let mut count = 0;

        for _i in 0..100 {
            count += step(&mut matrice);
        }

        assert_eq!(1656, count);
    }

    #[test]
    fn test_step() {
        let mut before: MatrixOctopus = parse_input(
            vec![
                "11111".to_string(),
                "19991".to_string(),
                "19191".to_string(),
                "19991".to_string(),
                "11111".to_string(),
            ],
            5,
        );

        let after: MatrixOctopus = parse_input(
            vec![
                "34543".to_string(),
                "40004".to_string(),
                "50005".to_string(),
                "40004".to_string(),
                "34543".to_string(),
            ],
            5,
        );

        println!("before {:?}", before);

        let changed = step(&mut before);
        assert_eq!(9, changed);
        assert_eq!(before, after);

        for _i in 0..100 {
            step(&mut before);
        }
    }
}
