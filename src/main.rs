#![allow(dead_code)]

mod day1;
mod day7;

fn main() {
//    day1::run1_2();
    day7::part2();
}

#[cfg(tests)]
mod tests {
    #[test]
    pub fn run1_1() {
        day1::run1_1();
    }
}
