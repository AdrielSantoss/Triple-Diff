use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use myers::myers_diff;

fn main() {
    let file_a = "src/fileA.txt";
    let file_b = "src/fileB.txt";

    let content_a = fs::read_to_string(file_a)
        .expect("Ocorreu um erro ao ler o FileA");

    let content_b = fs::read_to_string(file_b)
        .expect("Ocorreu um erro ao ler o FileB");

    let content_lines_a: Vec<&str> = content_a.lines().collect();
    let content_lines_b: Vec<&str> = content_b.lines().collect();

    let diffs = patience_diff(&content_lines_a, &content_lines_b);

    if !diffs.is_empty() {
        let mut patch_file = File::create("patch.txt")
            .expect("Ocorreu um erro ao gerar o arquivo patch.");

        for line in &diffs {
            writeln!(patch_file, "{line}")
                .unwrap_or_else(|_| panic!("Ocorreu um erro ao registrar a linha: {line}"));
        }
    }

    println!("Diffs: {diffs:?}");
}

fn patience_diff<'a>(content_lines_a: &'a [&'a str], content_lines_b: &'a [&'a str]) -> Vec<String> {
    let unique_lines_a = get_unique_lines(content_lines_a);
    let unique_lines_b = get_unique_lines(content_lines_b);

    let mut anchors = Vec::with_capacity(content_lines_a.len());
    let mut positions_b = Vec::with_capacity(content_lines_a.len());

    for &line in content_lines_a.iter() {
        if let (Some(&idx_a), Some(&idx_b)) = (unique_lines_a.get(line), unique_lines_b.get(line)) {
            anchors.push((line, idx_a, idx_b));
            positions_b.push(idx_b);
        }
    }

    if anchors.is_empty() && content_lines_a.len() >= 1 && content_lines_b.len() >= 1 {
        myers_diff(content_lines_a, content_lines_b);
        return Vec::new();
    }

    let lis_idx = get_lis_indices(&positions_b);
    let anchors_final: Vec<_> = lis_idx.iter().map(|&i| anchors[i]).collect();

    let mut diff = Vec::with_capacity(content_lines_a.len() + content_lines_b.len());
    let mut last_a = 0;
    let mut last_b = 0;

    for &(_, idx_a, idx_b) in &anchors_final {
        let sub_a = &content_lines_a[last_a..idx_a];
        let sub_b = &content_lines_b[last_b..idx_b];

        diff.extend(patience_diff(sub_a, sub_b));

        last_a = idx_a + 1;
        last_b = idx_b + 1;
    }

    let sub_a = &content_lines_a[last_a..];
    let sub_b = &content_lines_b[last_b..];

    for &removed_line in sub_a {
        diff.push(format!("-{removed_line}"));
    }
    for &added_line in sub_b {
        diff.push(format!("+{added_line}"));
    }

    return diff;
}

fn get_unique_lines<'a>(content_lines: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut freq = HashMap::with_capacity(content_lines.len());
    let mut result = HashMap::with_capacity(content_lines.len());

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

fn get_lis_indices(seq: &[usize]) -> Vec<usize> {
    if seq.is_empty() {
        return Vec::new();
    }

    let mut tails_vals = Vec::with_capacity(seq.len());
    let mut tails_indices = Vec::with_capacity(seq.len());
    let mut predecessors = vec![None; seq.len()];

    for (i, &x) in seq.iter().enumerate() {
        let pos = tails_vals.binary_search(&x).unwrap_or_else(|p| p);

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
    }

    let mut lis = Vec::new();
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