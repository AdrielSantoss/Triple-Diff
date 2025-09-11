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
    let vec = remove_comum_prefix_and_sufix(content_a, content_b);

    println!("new content a: {:?}", vec.0);
    println!("new content b: {:?}", vec.1);
}

fn remove_comum_prefix_and_sufix<'a>(content_a: &'a [&'a str], content_b: &'a [&'a str]) -> (&'a [&'a str], &'a [&'a str]) { 
    let mut last_prefix_position: usize = 0;
    let mut first_sufix_position: usize = 0;

    for (index, element_a) in content_a.iter().enumerate() {
        let element_b = &content_b[index];

        if element_a != element_b {
            break;
        }

        if element_a == element_b {
            last_prefix_position = index;
            continue;
        }
    }
    
    for index in (0..content_a.len()).rev() {
        let element_b = &content_b[index];
        let element_a = &content_a[index];

        if element_a != element_b {
            break;
        }

        if element_a == element_b {
            first_sufix_position = index;
            continue;
        }
    }

    return (&content_a[last_prefix_position + 1..first_sufix_position], &content_b[last_prefix_position + 1..first_sufix_position]);
}
