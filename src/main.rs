use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let file_a = "src/fileA.txt";
    let file_b = "src/fileB.txt";

    let content_a = fs::read_to_string(file_a)
        .expect("Ocorreu um erro ao ler o FileA");

    let content_b = fs::read_to_string(file_b)
        .expect("Ocorreu um erro ao ler o FileB");

    let content_lines_a: Vec<&str> = content_a.lines().collect();
    let content_lines_b: Vec<&str> = content_b.lines().collect();

    let unique_lines_a: Vec<&str> = get_unique_lines(&content_lines_a);
    let unique_lines_b: Vec<&str> = get_unique_lines(&content_lines_b);

    let mut anchors = Vec::new();

    let set_b: HashSet<_> = unique_lines_b.iter().cloned().collect();
    for &line_a in &unique_lines_a {
        if set_b.contains(line_a) {
            anchors.push(line_a);
    }
}

}

fn get_unique_lines<'a>(content_lines: &'a [&'a str]) -> Vec<&'a str> {
    let mut freq: HashMap<&'a str, usize> = HashMap::new();

    for &line in content_lines.iter() {
        *freq.entry(line).or_insert(0) += 1;
    }

    return content_lines
        .iter()
        .filter(|&&line| freq.get(line) == Some(&1))
        .map(|&line| line)
        .collect()
}