pub fn get_fields(line: &String) -> Vec<&str> {
    line.trim_start_matches("#Fields:")
        .split_whitespace()
        .collect()
}

pub fn get_values(line: &String) -> Vec<&str> {
    line.split_whitespace().collect()
}
