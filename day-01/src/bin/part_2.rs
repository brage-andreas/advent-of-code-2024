use day_01_lib::parse_input;
use library::read_input_into_lines;

fn occurrences_of<T: PartialEq>(list: &Vec<T>, element: &T) -> u32 {
    list
        .iter()
        .filter(|current_element| *current_element == element)
        .count() as u32
}

fn part_2() -> u32 {
    let input = read_input_into_lines();
    
    let (first_numbers, second_numbers) = parse_input(&input);

    first_numbers
        .into_iter()
        .map(|first_number| first_number * occurrences_of(&second_numbers, &first_number))
        .sum()
}

fn main() {
    println!("{}", part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_2(), 23963899)
    }
}
