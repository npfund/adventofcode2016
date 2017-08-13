use std::fs::File;
use std::io::Read;
use std::env;
use std::iter::FromIterator;
use std::collections::HashMap;

pub fn run1_1() {
    let deltas = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut current_delta:i32 = 0;
    let mut current_pos = (0, 0);

    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("src/day1/input.txt");
    let mut file = File::open(current_dir).unwrap();
    let mut contents = String::new();

    let _bytes = file.read_to_string(&mut contents);
    let directions:Vec<&str> = contents.trim().split(", ").collect();

    for direction in directions {
        let chars:Vec<char> = direction.chars().collect();
        let turn = chars[0];
        let steps = String::from_iter(chars[1..].iter()).parse::<i32>().unwrap();

        match turn {
            'R' => {
                current_delta = (current_delta + 1) % 4;
            },
            'L' => {
                current_delta = current_delta - 1;
                if current_delta < 0 {
                    current_delta = 3;
                }
            },
            _ => panic!("Weird turn")
        }

        current_pos.0 = current_pos.0 + deltas[current_delta as usize].0 * steps;
        current_pos.1 = current_pos.1 + deltas[current_delta as usize].1 * steps;
    }

    println!("{}", current_pos.0.abs() + current_pos.1.abs());
}

pub fn run1_2() {
    let deltas = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut current_delta: i32 = 0;
    let mut current_pos = (0, 0);
    let mut visited:HashMap<(i32, i32), bool> = HashMap::new();

    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("src/day1/input.txt");
    let mut file = File::open(current_dir).unwrap();
    let mut contents = String::new();

    let _bytes = file.read_to_string(&mut contents);
    let directions: Vec<&str> = contents.trim().split(", ").collect();

    for direction in directions {
        let chars: Vec<char> = direction.chars().collect();
        let turn = chars[0];
        let steps = String::from_iter(chars[1..].iter()).parse::<i32>().unwrap();

        match turn {
            'R' => {
                current_delta = (current_delta + 1) % 4;
            },
            'L' => {
                current_delta = current_delta - 1;
                if current_delta < 0 {
                    current_delta = 3;
                }
            },
            _ => panic!("Weird turn")
        }

        let mut found = false;
        for _i in 0..steps {
            current_pos.0 = current_pos.0 + deltas[current_delta as usize].0;
            current_pos.1 = current_pos.1 + deltas[current_delta as usize].1;

            if !visited.contains_key(&current_pos) {
                visited.insert(current_pos, true);
            } else {
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    println!("{}", current_pos.0.abs() + current_pos.1.abs());
}
