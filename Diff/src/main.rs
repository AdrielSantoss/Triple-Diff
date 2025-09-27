use myers::myers_diff;
use patience::patience_diff;
use utils::{write_patch_file, DiffOp};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <filePathA> <filePathB> [--patience]", args[0]);
        std::process::exit(1);
    }

    let file_a = &args[1];
    let file_b = &args[2];

    let use_patience = args.iter().any(|arg| arg == "--patience");

    let content_a = fs::read_to_string(file_a)
        .expect(&format!("Error reading file {}", file_a));
    let content_b = fs::read_to_string(file_b)
        .expect(&format!("Error reading file {}", file_b));

    let lines_a: Vec<&str> = content_a.lines().collect();
    let lines_b: Vec<&str> = content_b.lines().collect();

    if lines_a.is_empty() && lines_a.is_empty() {
        println!("No differences found");
    }

    let edits: Vec<DiffOp> = if use_patience {
        patience_diff(&lines_a, &lines_b)
    } else {
        myers_diff(&lines_a, &lines_b)
    };

    if edits.is_empty() {
        println!("No differences found");
        return;
    }

    write_patch_file(&edits, "patch.diff");
    println!("Diff written to patch.diff");
}
