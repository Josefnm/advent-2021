use std::fs;

mod day1;

fn main() {
    day1::part1();
    day1::part2();
}

pub fn get_text(day: &str) -> String {
    return fs::read_to_string(format!("{}{}{}", "src/", day, "/input.txt"))
        .expect("error reading the file")
        .trim()
        .to_string();
}

pub trait StringExt {
    fn split_by_line(self) -> Vec<String>;
}

impl StringExt for String {
    fn split_by_line(self) -> Vec<String> {
        return Vec::from_iter(self.split('\n').map(String::from));
    }
}
