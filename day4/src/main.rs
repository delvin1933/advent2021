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

type BingoLine = Vec<BingoNumber>;

type Board = Vec<BingoLine>;

trait BingoBoard {
    fn check_number(&mut self, drawn: i32);
    fn win(&self) -> bool;
    fn get_final_number(&self) -> i32;
}

impl BingoBoard for Board {
    fn check_number(&mut self, drawn: i32) {
        for line in self {
            for bnumber in line {
                bnumber.check(drawn)
            }
        }
    }

    fn win(&self) -> bool {
        let mut winner: bool;
        for line in self {
            winner = true;
            for bnumber in line {
                winner = winner && bnumber.checked;
            }
            if winner {
                return winner;
            }
        }

        for i in 0..5 {
            winner = true;
            for line in self {
                winner = winner && line[i].checked
            }
            if winner {
                return winner;
            }
        }
        false
    }

    fn get_final_number(&self) -> i32 {
        let mut res = 0;
        for line in self {
            for bnumber in line {
                if bnumber.checked == false {
                    res = res + bnumber.value;
                }
            }
        }
        return res;
    }
}

#[derive(Default, Debug)]
struct BingoNumber {
    pub value: i32,
    pub checked: bool,
}

impl BingoNumber {
    fn new(value: i32) -> Self {
        BingoNumber {
            value,
            checked: false,
        }
    }
    fn check(&mut self, other: i32) {
        if self.value == other {
            self.checked = true
        }
    }
}

impl From<i32> for BingoNumber {
    fn from(value: i32) -> Self {
        BingoNumber::new(value)
    }
}

fn main() {
    let lines = read_lines("./input.txt").unwrap_or_default();

    let tirages: Vec<i32> = match lines.first() {
        Some(line) => line.split(',').map(|e| e.parse::<i32>().unwrap()).collect(),
        None => vec![0],
    };

    let mut boards: Vec<Board> = Default::default();

    for raw_board in lines[1..].chunks(6) {
        let mut array: Board = Default::default();

        for line in raw_board[1..].iter() {
            let numbers: Vec<BingoNumber> = line
                .split_whitespace()
                .map(|e| BingoNumber::new(e.parse::<i32>().unwrap()))
                .collect();

            array.push(numbers.into());
        }
        boards.push(array);
    }
    for mut board in boards {
        for tirage in &tirages {
            board.check_number(*tirage);

            println!("Tirage {:?}", tirage);
            if board.win() {
                println!("{:?}", board);
                println!("{}", tirage);
                println!("Final number {}", board.get_final_number());
                println!("Final number mult {}", board.get_final_number() * tirage);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_win() {
        let mut array: Board = Default::default();
        array.push(Vec::from([
            BingoNumber::new(1),
            BingoNumber::new(2),
            BingoNumber::new(3),
            BingoNumber::new(4),
            BingoNumber::new(5),
        ]));
        array.push(Vec::from([
            BingoNumber::new(1),
            BingoNumber::new(2),
            BingoNumber::new(3),
            BingoNumber::new(4),
            BingoNumber::new(5),
        ]));
        array.push(Vec::from([
            BingoNumber::new(1),
            BingoNumber::new(2),
            BingoNumber::new(3),
            BingoNumber::new(4),
            BingoNumber::new(5),
        ]));
        array.push(Vec::from([
            BingoNumber::new(1),
            BingoNumber::new(2),
            BingoNumber::new(3),
            BingoNumber::new(4),
            BingoNumber::new(5),
        ]));
        array.push(Vec::from([
            BingoNumber::new(1),
            BingoNumber::new(2),
            BingoNumber::new(3),
            BingoNumber::new(4),
            BingoNumber::new(5),
        ]));

        for number in [1, 2, 3, 4, 5] {
            array.check_number(number);
        }

        assert_eq!(true, array.win());
        let mut array: Board = Default::default();
        array.push(Vec::from([
            BingoNumber::new(1),
            BingoNumber::new(2),
            BingoNumber::new(3),
            BingoNumber::new(4),
            BingoNumber::new(5),
        ]));
        array.push(Vec::from([
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
        ]));
        array.push(Vec::from([
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
        ]));
        array.push(Vec::from([
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
        ]));
        array.push(Vec::from([
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
        ]));

        for number in [1, 2, 3, 4, 6] {
            array.check_number(number);
        }
        assert_eq!(false, array.win());

        let mut array: Board = Default::default();
        array.push(Vec::from([
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(1),
        ]));
        array.push(Vec::from([
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(2),
        ]));
        array.push(Vec::from([
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(3),
        ]));
        array.push(Vec::from([
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(4),
        ]));
        array.push(Vec::from([
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(5),
        ]));
        for number in [1, 2, 3, 4, 5] {
            array.check_number(number);
        }
        assert_eq!(true, array.win());

        let mut array: Board = Default::default();
        array.push(Vec::from([
            BingoNumber::new(1),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
        ]));
        array.push(Vec::from([
            BingoNumber::new(2),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
        ]));
        array.push(Vec::from([
            BingoNumber::new(3),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
        ]));
        array.push(Vec::from([
            BingoNumber::new(4),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
        ]));
        array.push(Vec::from([
            BingoNumber::new(5),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
            BingoNumber::new(9),
        ]));
        for number in [1, 2, 3, 4, 6] {
            array.check_number(number);
        }
        assert_eq!(false, array.win());

        let mut array: Board = Default::default();
        array.push(Vec::from([
            BingoNumber::new(22),
            BingoNumber::new(59),
            BingoNumber::new(7),
            BingoNumber::new(10),
            BingoNumber::new(96),
        ]));
        array.push(Vec::from([
            BingoNumber::new(33),
            BingoNumber::new(36),
            BingoNumber::new(96),
            BingoNumber::new(55),
            BingoNumber::new(23),
        ]));
        array.push(Vec::from([
            BingoNumber::new(13),
            BingoNumber::new(85),
            BingoNumber::new(18),
            BingoNumber::new(29),
            BingoNumber::new(28),
        ]));
        array.push(Vec::from([
            BingoNumber::new(75),
            BingoNumber::new(46),
            BingoNumber::new(83),
            BingoNumber::new(73),
            BingoNumber::new(58),
        ]));
        array.push(Vec::from([
            BingoNumber::new(34),
            BingoNumber::new(40),
            BingoNumber::new(87),
            BingoNumber::new(56),
            BingoNumber::new(98),
        ]));
        for number in [
            6, 69, 28, 50, 36, 84, 49, 13, 48, 90, 1, 33, 71, 0, 94, 59, 53, 58, 60, 96, 30, 34,
            29, 91, 11, 41, 77, 95, 17, 80, 85, 93, 7, 9, 74, 89, 18,
        ] {
            array.check_number(number);
        }
        assert_eq!(true, array.win());
    }
}
