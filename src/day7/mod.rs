use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

pub fn part1() {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("src/day7/input.txt");
    let file = File::open(current_dir).unwrap();

    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let address = line.unwrap();
        let bytes = address.bytes();

        let mut found_abba = false;
        let mut inside = false;
        let mut found_interior_abba = false;
        let mut quartet: Vec<u8> = Vec::new();
        for byte in bytes {
            match byte {
                b'[' => {
                    quartet.truncate(0);
                    inside = true;
                },
                b']' => {
                    quartet.truncate(0);
                    inside = false;
                },
                _ => {
                    quartet.push(byte);

                    if quartet.len() > 4 {
                        let _old = quartet.remove(0);
                    }

                    if quartet.len() == 4 {
                        if quartet[0] != quartet[1] && quartet[0] == quartet[3] && quartet[1] == quartet[2] {
                            if inside {
                                found_interior_abba = true;
                            } else {
                                found_abba = true;
                            }
                        }
                    }
                }
            }
        }

        if found_abba && !found_interior_abba {
            total = total + 1;
        }
    }

    println!("{}", total);
}
