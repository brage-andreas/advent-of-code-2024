use regex::Regex;
use library::capture_parse;

pub fn sum_enabled_mul_instructions(input: &Vec<String>) -> u32 {
    let mul_regex = Regex::new(r"^\((?<first_number>\d{1,3}),(?<second_number>\d{1,3})\)").unwrap();

    input.iter()
        .flat_map(|line| line.split("mul"))
        .filter_map(|part| mul_regex.captures(part))
        .map(|capture| {
            let first_number: u32 = capture_parse(&capture, "first_number");
            let second_number: u32 = capture_parse(&capture, "second_number");

            first_number * second_number
        })
        .sum()
}
