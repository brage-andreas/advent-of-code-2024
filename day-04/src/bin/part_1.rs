use library::read_input_into_lines;
use std::cmp::max;

type Board = Vec<Vec<String>>;

const WORD: [&str; 4] = ["X", "M", "A", "S"];

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn get_next(
    (x, y): (usize, usize),
    (x_coefficient, y_coefficient): &(i32, i32),
    (max_x, max_y): (usize, usize),
    iteration: usize,
) -> Option<(usize, usize)> {
    let next_x = x as i32 + x_coefficient * iteration as i32;
    let next_y = y as i32 + y_coefficient * iteration as i32;

    if (0..max_x as i32).contains(&next_x) || (0..max_y as i32).contains(&next_y) {
        return None;
    }

    Some((next_x as usize, next_y as usize))
}

// extremely inefficient
fn check_cross(board: &Board, (x, y): (u32, u32)) -> Vec<Vec<(usize, usize)>> {
    let max_x = board[0].len();
    let max_y = board.len();

    let mut result = Vec::new();

    for y in 0..max_y {
        for x in 0..max_x {
            for x_and_y_coefficients in DIRECTIONS.iter() {
                let mut lol = Vec::new();
                
                for i in 0..WORD.len() {
                    let Some((next_x, next_y)) = get_next(
                        (x, y),
                        x_and_y_coefficients,
                        (max_x, max_y),
                        i
                    ) else {
                        break;
                    };
    
                    if board[next_y][next_x] != WORD[i] {
                        break;
                    }
                    
                    lol.push((next_x, next_y));
    
                    if lol.len() == WORD.len() {
                        result.push(lol.clone());
                    }
                }
            }
        }
    }

    result
}

fn part_1() -> u32 {
    let input = read_input_into_lines();

    0
}

fn main() {
    println!("{}", part_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(), 0)
    }
}
