use std::{env, fs};
use myers::{myers_diff, write_patch_file};

fn main() {
    // Captura argumentos da linha de comando
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <fileA> <fileB>", args[0]);
        std::process::exit(1);
    }

    let file_a = &args[1];
    let file_b = &args[2];

    let content_a = fs::read_to_string(file_a)
        .expect(&format!("Error reading file {}", file_a));

    let content_b = fs::read_to_string(file_b)
        .expect(&format!("Error reading file {}", file_b));

    let lines_a: Vec<&str> = content_a.lines().collect();
    let lines_b: Vec<&str> = content_b.lines().collect();

    let edits = myers_diff(&lines_a, &lines_b);

    if edits.is_empty() {
        println!("No differences found");
    } else {
        write_patch_file(&edits, "patch.diff");
        println!("Diff written to patch.diff");
    }
}