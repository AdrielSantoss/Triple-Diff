use std::{collections::HashMap, fs};

fn main() {
    let file_a = "src/fileA.txt";
    let file_b = "src/fileB.txt";

    let content_a = fs::read_to_string(file_a)
        .expect("Ocorreu um erro ao ler o FileA");

    let content_b = fs::read_to_string(file_b)
        .expect("Ocorreu um erro ao ler o FileB");

    let content_lines_a: Vec<&str> = content_a.split("\n").collect();
    let content_lines_b: Vec<&str> = content_b.split("\n").collect();

    let mut content_dictionary_a: HashMap<String, usize> = HashMap::new();
    let mut content_dictionary_b: HashMap<String, usize> = HashMap::new();

    for i in 0..content_lines_a.len() {
        if content_dictionary_a.contains_key(content_lines_a[i]) {
            let value = content_dictionary_a.get(content_lines_a[i]).unwrap();
            content_dictionary_a.insert(content_lines_a[i].to_string(), value + 1);

            continue;
        }

        content_dictionary_a.insert(content_lines_a[i].to_string(), 1);
    }
}
