use regex::Regex;
use library::read_input_into_lines;

fn part_1() -> u32 {
    let input = read_input_into_lines();

    let regex = Regex::new(r"(?<first_number>\d{5})\s+(?<second_number>\d{5})").unwrap();

    let mut first_numbers: Vec<u32> = input
        .iter()
        .filter_map(|line| {
            regex.captures(line).map(|capture| {
                capture.name("first_number").unwrap().as_str().parse::<u32>().unwrap()
            })
        }).collect();

    let mut second_numbers: Vec<u32> = input
        .iter()
        .filter_map(|line| {
            regex.captures(line).map(|capture| {
                capture.name("second_number").unwrap().as_str().parse::<u32>().unwrap()
            })
        })
        .collect();

    first_numbers.sort();
    second_numbers.sort();

    first_numbers
        .iter()
        .zip(second_numbers.iter())
        .map(|(a, b)| a.abs_diff(*b))
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
        assert_eq!(part_1(), 2970687)
    }
}
