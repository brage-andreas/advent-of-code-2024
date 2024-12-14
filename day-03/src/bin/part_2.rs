use day_03_lib::sum_enabled_mul_instructions;
use library::read_input_into_lines;
use regex::Regex;

const DELIMITER: &str = "[[DELIMITER]]";

fn extract_enabled_parts(input: &Vec<String>) -> Vec<String> {
    let dont_to_do_regex = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let dont_to_eol_regex = Regex::new(r"don't\(\).*").unwrap();

    let mut cleaned_input = input.join(DELIMITER); // To keep state

    // Delete everything after `don't()` until `do()`
    while dont_to_do_regex.is_match(&cleaned_input) {
        cleaned_input = dont_to_do_regex.replace_all(&cleaned_input, "").to_string();
    }
    
    dont_to_eol_regex
        .replace_all(&cleaned_input, "") // Delete everything after `don't()` until end-of-line
        .split(DELIMITER)
        .map(|part| part.to_string())
        .collect()
}

fn part_2(input: &Vec<String>) -> u32 {
    let enabled_parts = extract_enabled_parts(&input);

    sum_enabled_mul_instructions(&enabled_parts)
}

fn main() {
    let lines = read_input_into_lines();

    println!("{}", part_2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_parts_test() {
        let vec = vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ];

        assert_eq!(
            extract_enabled_parts(&vec),
            vec!["xmul(2,4)&mul[3,7]!^?mul(8,5))"]
        );
    }

    #[test]
    fn part_2_test() {
        let lines = read_input_into_lines();

        assert_eq!(part_2(&lines), 104083373)
    }
}
