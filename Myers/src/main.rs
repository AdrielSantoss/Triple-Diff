use std::fs;

use myers::{remove_comum_prefix_and_suffix, myers_diff};

fn main() {
    let file_a = "src/fileA.txt";
    let file_b = "src/fileB.txt";

    let content_a = fs::read_to_string(file_a)
        .expect("Ocorreu um erro ao ler o FileA");

    let content_b = fs::read_to_string(file_b)
        .expect("Ocorreu um erro ao ler o FileB");

    let content_lines_a: Vec<&str> = content_a.lines().collect();
    let content_lines_b: Vec<&str> = content_b.lines().collect();

    let (mid_a, mid_b) = remove_comum_prefix_and_suffix(&content_lines_a, &content_lines_b);

    println!("Meio de A: {:?}", mid_a);
    println!("Meio de B: {:?}", mid_b);

    myers_diff(mid_a, mid_b);
}

