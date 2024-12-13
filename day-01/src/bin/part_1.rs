use day_01_lib::parse_input;
use library::read_input_into_lines;

fn part_1() -> u32 {
    let input = read_input_into_lines();

    let (mut first_numbers, mut second_numbers) = parse_input(&input);

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
