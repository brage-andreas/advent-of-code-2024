use day_02_lib::get_predicate;
use library::{parse_line, read_input_into_lines};

const MAX_DIFFERENCE: u32 = 3;

fn is_safe_sequence(sequence: Vec<u32>) -> u32 {
    let predicate = get_predicate(&sequence);

    let mut has_errored = false;

    for windows in sequence.windows(2) {
        let (a, b) = (windows[0], windows[1]);

        if !predicate(a, b) || a.abs_diff(b) > MAX_DIFFERENCE {
            if has_errored {
                return 0
            }

            has_errored = true;
        }
    }

    1
}

fn part_2() -> u32 {
    let input = read_input_into_lines();

    input.iter().map(|line| is_safe_sequence(parse_line(line))).sum()
}

fn main() {
    println!("{}", part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_2(), 626)
    }
}
