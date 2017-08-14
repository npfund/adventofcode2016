use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::str::FromStr;

struct Screen {
    display: Vec<Vec<bool>>,
}

impl Screen {
    fn new() -> Screen {
        let mut display = Vec::new();
        for _i in 0..6 {
            display.push(vec![false; 50]);
        }

        return Screen {
            display: display
        }
    }

    fn rect(&mut self, col: usize, row: usize) {
        for y in 0..row {
            for x in 0..col {
                self.display[y][x] = true;
            }
        }
    }

    fn rotate_column(&mut self, col: usize, amount: usize) {
        for _i in 0..amount {
            let last = self.display[self.display.len() - 1][col];
            for y in (1..self.display.len()).rev() {
                self.display[y][col] = self.display[y-1][col];
            }
            self.display[0][col] = last;
        }
    }

    fn rotate_row(&mut self, row: usize, amount: usize) {
        for _i in 0..amount {
            let old = self.display[row].pop().unwrap();
            self.display[row].insert(0, old);
        }
    }

    fn count_lit(&self) -> usize {
        let mut total: usize = 0;
        for row in &self.display {
            total = total + row.iter().filter(|x| **x).count();
        }

        return total;
    }

    fn print(&self) {
        for row in &self.display {
            for col in row {
                if *col {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

pub fn parts1_and_2() {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("src/day8/input.txt");
    let file = File::open(current_dir).unwrap();

    let reader = BufReader::new(file);

    let mut screen = Screen::new();

    for line in reader.lines() {
        let command = line.unwrap();
        let parts: Vec<&str> = command.trim().split_whitespace().collect();

        match parts[0] {
            "rect" => {
                let (col_str, row_str) = parts[1].split_at(parts[1].find('x').unwrap());
                let col: usize = String::from_str(col_str).unwrap().parse::<usize>().unwrap();
                let row: usize = String::from_str(&(row_str[1..])).unwrap().parse::<usize>().unwrap();
                screen.rect(col, row);
            },
            "rotate" => {
                let (_coord, coord_str) = parts[2].split_at(parts[2].find('=').unwrap());
                let coord: usize = String::from_str(&(coord_str[1..])).unwrap().parse::<usize>().unwrap();
                let amount: usize = String::from_str(parts[4]).unwrap().parse::<usize>().unwrap();

                match parts[1] {
                    "row" => screen.rotate_row(coord, amount),
                    "column" => screen.rotate_column(coord, amount),
                    _ => panic!("Unknown rotate {}", parts[1])
                }
            }
            _ => panic!("Unknown command {}", parts[0])
        }
    }

    screen.print();
    println!("{}", screen.count_lit());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rect() {
        let mut screen = Screen::new();
        screen.rect(2, 1);
        assert_eq!(true, screen.display[0][0]);
        assert_eq!(true, screen.display[0][1]);
    }

    #[test]
    fn count_lit() {
        let mut screen = Screen::new();
        screen.rect(1, 1);
        assert_eq!(1, screen.count_lit());
    }

    #[test]
    fn rotate_row() {
        let mut screen = Screen::new();
        screen.rect(2, 1);
        screen.rotate_row(0, 5);
        assert_eq!(true, screen.display[0][5]);
        assert_eq!(true, screen.display[0][6]);
    }

    #[test]
    fn rotate_column() {
        let mut screen = Screen::new();
        screen.rect(1, 2);
        screen.rotate_column(0, 5);
        assert_eq!(true, screen.display[0][0]);
        assert_eq!(true, screen.display[5][0]);
    }
}
