use std::fs;

use myers::{myers_diff, remove_comum_prefix_and_suffix, write_patch_file, DiffOp};

fn main() {
    let file_a = "src/fileA.txt";
    let file_b = "src/fileB.txt";

    let content_a = fs::read_to_string(file_a)
        .expect("Ocorreu um erro ao ler o FileA");

    let content_b = fs::read_to_string(file_b)
        .expect("Ocorreu um erro ao ler o FileB");

    let content_lines_a: Vec<&str> = content_a.lines().collect();
    let content_lines_b: Vec<&str> = content_b.lines().collect();

    let (prefix, mid_a, mid_b, suffix_a) =
        remove_comum_prefix_and_suffix(&content_lines_a, &content_lines_b);

    let mut edits: Vec<DiffOp> = Vec::new();

    for line in prefix {
        edits.push(DiffOp::Match(line));
    }

    edits.extend(myers_diff(mid_a, mid_b));

    for line in suffix_a {
        edits.push(DiffOp::Match(line));
    }

    write_patch_file(&edits, "patch.diff");
}

