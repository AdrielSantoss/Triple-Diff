use std::{collections::{HashSet}, fs};

fn main() {
    let file_a = "src/fileA.txt";
    let file_b = "src/fileB.txt";

    let content_a = fs::read_to_string(file_a)
        .expect("Ocorreu um erro ao ler o FileA");

    let content_b = fs::read_to_string(file_b)
        .expect("Ocorreu um erro ao ler o FileB");

    let content_lines_a: Vec<&str> = content_a.split("\n").collect();
    let content_lines_b: Vec<&str> = content_b.split("\n").collect();

    let mut unique_lines_a: Vec<&str> = get_unique_lines(content_lines_a);
    let mut unique_lines_b: Vec<&str> = get_unique_lines(content_lines_b);
}


fn get_unique_lines(content_lines: Vec<&str>) -> Vec<&str> {
    let mut unique_lines = Vec::new();
    let mut seen = HashSet::new();

    for item in content_lines {
        if seen.insert(item) {
            unique_lines.push(item);
        }
    }

    return unique_lines;
}