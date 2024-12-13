use regex::Regex;

fn extract_capture(line: &str, regex: &Regex, capture_group_name: &str) -> Option<u32> {
    regex.captures(line).map(|capture| {
        let group_capture = capture.name(capture_group_name).unwrap();
        
        group_capture.as_str().parse::<u32>().unwrap()
    })
}

fn extract_all_captures(input: &Vec<String>, regex: &Regex, capture_group_name: &str) -> Vec<u32> {
    input
        .iter()
        .filter_map(|line| extract_capture(line, regex, capture_group_name))
        .collect()
}

pub fn parse_input(input: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let regex = Regex::new(r"(?<first_number>\d{5})\s+(?<second_number>\d{5})").unwrap();
    
    let first_numbers: Vec<u32> = extract_all_captures(input, &regex, "first_number");
    let second_numbers: Vec<u32> = extract_all_captures(input, &regex, "second_number");

    (first_numbers, second_numbers)
}