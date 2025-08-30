use std::{collections::{HashMap, HashSet}, fs, ops::Index};

fn main() {
    let file_a = "src/fileA.txt";
    let file_b = "src/fileB.txt";

    let content_a = fs::read_to_string(file_a)
        .expect("Ocorreu um erro ao ler o FileA");

    let content_b = fs::read_to_string(file_b)
        .expect("Ocorreu um erro ao ler o FileB");

    let content_lines_a: Vec<&str> = content_a.lines().collect();
    let content_lines_b: Vec<&str> = content_b.lines().collect();

    let unique_lines_a: HashMap<&str, usize> = get_unique_lines(&content_lines_a);
    let unique_lines_b: HashMap<&str, usize> = get_unique_lines(&content_lines_b);

    let mut anchors: Vec<(&str, usize)> = Vec::new();

    for &line in content_lines_a.iter() {
        if let Some(&pos_b) = unique_lines_b.get(line) {
            if unique_lines_a.contains_key(line) {
                anchors.push((line, pos_b));
            }
        }
    }
}

fn get_unique_lines<'a>(content_lines: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut freq: HashMap<&'a str, usize> = HashMap::new();
    let mut result: HashMap<&'a str, usize> = HashMap::new();

    for (i, &line) in content_lines.iter().enumerate() {    
        let count = freq.entry(line).or_insert(0);
        *count += 1;

        if *count == 1 {
            result.insert(line, i);
        } else {
            result.remove(line);
        }
    }

    return result;
}