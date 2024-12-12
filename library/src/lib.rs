use std::fs;

fn get_input_file_path_from_day(day: u8) -> String {
    format!("day-{:02}/src/input.txt", day)
}

fn read_file_relative_from_workspace_root(relative_path: &str) -> String {
    match fs::read_to_string(&relative_path) {
        Ok(result) => result,
        Err(error) => panic!(
            "Could not read from local file `{}`\n  Message: \"{:?}\"",
            &relative_path, &error
        ),
    }
}

pub fn read_input(day: u8) -> String {
    let input_file_path = get_input_file_path_from_day(day);

    read_file_relative_from_workspace_root(&input_file_path)
}

pub fn read_input_into_lines() -> Vec<String> {
    read_input().split("\n").map(|line| line.to_string()).collect()
}

pub fn read_file(crate_relative_path: String) -> String {
    let path = construct_file_path(crate_relative_path);
    
    read_file_relative_from_crate(path)
}