use std::fs;
use myers::{myers_diff, write_patch_file};

fn main() {
    let file_a = "src/fileA.txt";
    let file_b = "src/fileB.txt";

    let content_a = fs::read_to_string(file_a)
        .expect("Ocorreu um erro ao ler o FileA");

    let content_b = fs::read_to_string(file_b)
        .expect("Ocorreu um erro ao ler o FileB");

    let lines_a: Vec<&str> = content_a.lines().collect();
    let lines_b: Vec<&str> = content_b.lines().collect();

    let edits = myers_diff(&lines_a, &lines_b);

    write_patch_file(&edits, "patch.diff");
}

