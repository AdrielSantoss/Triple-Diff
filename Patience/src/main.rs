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

    let unique_lines_a: HashMap<&str, usize> = get_unique_lines(&content_lines_a);
    let unique_lines_b: HashMap<&str, usize> = get_unique_lines(&content_lines_b);

    let mut anchors: Vec<&str> = Vec::new();
    let mut positions_b: Vec<usize> = Vec::new();

    for &line in content_lines_a.iter() {
        if let Some(&pos_b) = unique_lines_b.get(line) {
            if unique_lines_a.contains_key(line) {
                anchors.push(line);
                positions_b.push(pos_b);
            }
        }
    }

    let mut anchorsFinal: Vec<&str> = Vec::new();
    let mut lastIndex: i32 = -1;

    for (position_a, line_a) in content_lines_a.iter().enumerate() {
        let mut anchorPos: usize = 0;

        if !anchors.contains(line_a) {
            continue;
        }
        else {
            anchorPos = anchors.iter().position(|x| x == line_a).unwrap();
        }

        let position_b = positions_b[anchorPos];
        
        if lastIndex == -1 {
            if position_b > position_a {
                continue;
            }
            else {
                anchorsFinal.push(line_a);
                lastIndex = position_a as i32;
                continue;
            }   
        }

        let lastIndexUsized: usize = lastIndex as usize;

        if position_b > lastIndexUsized && position_b >= position_a {
            anchorsFinal.push(line_a);
            lastIndex = position_a as i32;
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