use library::{parse_line, read_input_into_lines};

const MAX_DIFFERENCE: u32 = 3;

fn is_safe_sequence(sequence: Vec<u32>) -> u32 {
    let predicate = if sequence[0] < sequence[1] {
        |a: u32, b: u32| a < b
    } else {
        |a: u32, b: u32| a > b
    };

    for index in 1..sequence.len() {
        let a = sequence[index - 1];
        let b = sequence[index];
        
        if !predicate(a, b) || a.abs_diff(b) > MAX_DIFFERENCE {
            return 0;
        }
    }

    1
}

fn part_1() -> u32 {
    let input = read_input_into_lines();
    
    input.iter().map(|line| is_safe_sequence(parse_line(line))).sum()
}

fn main() {
    println!("{}", part_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(), 585)
    }
}
