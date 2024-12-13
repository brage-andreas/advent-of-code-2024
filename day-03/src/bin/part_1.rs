use regex::Regex;
use library::{capture_parse, read_input_into_lines};

fn part_1() -> u32 {
    let regex = Regex::new(r"^\((?<first_number>\d{1,3}),(?<second_number>\d{1,3})\)").unwrap();

    read_input_into_lines()
        .iter()
        .flat_map(|line| line.split("mul"))
        .filter_map(|part| regex.captures(part))
        .map(|capture| {
            let first_number: u32 = capture_parse(&capture, "first_number");
            let second_number: u32 = capture_parse(&capture, "second_number");

            first_number * second_number
        })
        .sum()
}

fn main() {
    println!("{}", part_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(), 192767529)
    }
}
