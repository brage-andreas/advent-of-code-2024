use day_03_lib::sum_enabled_mul_instructions;
use library::read_input_into_lines;

fn part_1() -> u32 {
    let input = read_input_into_lines();

    sum_enabled_mul_instructions(&input)
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
