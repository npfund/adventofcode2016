use std::fs::File;
use std::io::Read;
use std::env;

pub fn part1() {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("src/day9/input.txt");
    let mut file = File::open(current_dir).unwrap();
    let mut contents = String::new();

    let _bytes = file.read_to_string(&mut contents);

    let mut output = String::new();
    let mut bytes = contents.trim().bytes();
    while let Some(byte) = bytes.next() {
        match byte {
            b'(' => {
                let mut buffer_string = String::new();
                let mut next = bytes.next().unwrap();

                while next != b'x' {
                    buffer_string.push(next as char);
                    next = bytes.next().unwrap();
                }

                let buffer_size = buffer_string.parse::<usize>().unwrap();

                let mut repetition_string = String::new();
                next = bytes.next().unwrap();

                while next != b')' {
                    repetition_string.push(next as char);
                    next = bytes.next().unwrap();
                }

                let total_repetition = repetition_string.parse::<usize>().unwrap();

                let mut buffer = String::new();
                for _i in 0..buffer_size {
                    buffer.push(bytes.next().unwrap() as char);
                }

                for _i in 0..total_repetition {
                    output.push_str(&buffer);
                }
            },
            _ => {
                output.push(byte as char);
            }
        }
    }

    println!("{}", output.len());
}
