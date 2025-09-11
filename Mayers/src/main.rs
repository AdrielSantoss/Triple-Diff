use std::{
    fs::{self}
};

fn main() {
    let file_a = "src/fileA.txt";
    let file_b = "src/fileB.txt";

    let content_a = fs::read_to_string(file_a)
        .expect("Ocorreu um erro ao ler o FileA");

    let content_b = fs::read_to_string(file_b)
        .expect("Ocorreu um erro ao ler o FileB");

    let content_lines_a: Vec<&str> = content_a.lines().collect();
    let content_lines_b: Vec<&str> = content_b.lines().collect();

    mayers_diff(&content_lines_a, &content_lines_b);
}

fn mayers_diff(content_a: &[&str], content_b: &[&str]) {
    let vec = remove_comum_prefix(content_a, content_b);

    println!("new content a: {:?}", vec.0);
    println!("new content b: {:?}", vec.1);
}

fn remove_comum_prefix(content_a: &[&str], content_b: &[&str]) -> (Vec<String>, Vec<String>) { 
    let mut new_content_a: Vec<String> = Vec::with_capacity(content_a.len());
    let mut new_content_b: Vec<String> = Vec::with_capacity(content_b.len());

    for (index, element_a) in content_a.iter().enumerate() {
        if index >= content_b.len() {
            new_content_a.push(element_a.to_string());
            continue;
        }

        let element_b = &content_b[index];

        if element_a == element_b {
            continue;
        }

        new_content_a.push(element_a.to_string());
        new_content_b.push(element_b.to_string());
    }

    if content_b.len() > content_a.len() {
        for element_b in &content_b[content_a.len()..] {
            new_content_b.push(element_b.to_string());
        }
    }

    return (new_content_a, new_content_b);
}
