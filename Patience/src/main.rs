use std::{collections::HashMap, fs};

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

    let lis_idx = lis_indices(&positions_b);

    let anchors_final: Vec<&str> = lis_idx.iter().map(|&i| anchors[i]).collect();

    println!("Anchors (candidatas): {:?}", anchors);
    println!("Positions in B: {:?}", positions_b);
    println!("LIS indices: {:?}", lis_idx);
    println!("Anchors final: {:?}", anchors_final);
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

    result
}

fn lis_indices(seq: &[usize]) -> Vec<usize> {
    let n = seq.len();
    if n == 0 {
        return Vec::new();
    }

    let mut tails_vals: Vec<usize> = Vec::new();
    let mut tails_indices: Vec<usize> = Vec::new();
    let mut predecessors: Vec<Option<usize>> = vec![None; n];

    for (i, &x) in seq.iter().enumerate() {
        let pos = match tails_vals.binary_search(&x) {
            Ok(p) => p,
            Err(p) => p,
        };

        if pos == tails_vals.len() {
            tails_vals.push(x);
            tails_indices.push(i);
        } else {
            tails_vals[pos] = x;
            tails_indices[pos] = i;
        }

        if pos > 0 {
            predecessors[i] = Some(tails_indices[pos - 1]);
        }
        else {
            predecessors[i] = None;
        }
    }

    let mut lis: Vec<usize> = Vec::new();
    if let Some(&last_index) = tails_indices.last() {
        let mut k = Some(last_index);
        while let Some(idx) = k {
            lis.push(idx);
            k = predecessors[idx];
        }
        lis.reverse();
    }

    return lis;
}
