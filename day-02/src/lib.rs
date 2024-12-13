pub fn get_predicate(sequence: &Vec<u32>) -> fn(a: u32, b: u32) -> bool {
    if sequence[0] < sequence[1] {
        |a: u32, b: u32| a < b
    } else {
        |a: u32, b: u32| a > b
    }
}