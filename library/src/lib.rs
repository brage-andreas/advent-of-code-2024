use std::fmt::Debug;
use std::str::FromStr;

fn construct_file_path(crate_relative_path: String) -> String {
    let base_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    format!("{}/{}", base_path, crate_relative_path.replace(r"^/", ""))
}

fn get_input_file_path() -> String {
    construct_file_path("src/input.txt".to_string())
}

fn read_file_relative_from_crate(relative_path: String) -> String {
    std::fs::read_to_string(&relative_path).unwrap_or_else(|error| {
        panic!(
            "Could not read from file `{}`\n  Error: {:?}",
            relative_path, error
        )
    })
}

pub fn read_input() -> String {
    let input_file_path = get_input_file_path();

    read_file_relative_from_crate(input_file_path)
}

pub fn read_input_into_lines() -> Vec<String> {
    read_input().split("\n").map(|line| line.to_string()).collect()
}

pub fn read_file(crate_relative_path: String) -> String {
    let path = construct_file_path(crate_relative_path);
    
    read_file_relative_from_crate(path)
}

pub fn parse_line<T: FromStr>(line: &String) -> Vec<T> where <T as FromStr>::Err: Debug {
    line.split_whitespace().map(|number| number.parse::<T>().unwrap()).collect()
}
