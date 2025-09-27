use std::fs::File;
use std::io::Write;
use std::{
    env, fs::{self}
};
use colored::*;

pub enum DiffOp<'a> {
    Match(&'a str),
    Insert(&'a str),
    Delete(&'a str),
}

pub fn get_content_files<'a>() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <filePathA> <filePathB>", args[0]);
        std::process::exit(1);
    }

    let file_a = &args[1];
    let file_b = &args[2];

    let content_a = fs::read_to_string(file_a)
        .expect(&format!("Error reading file {}", file_a));
    let content_b = fs::read_to_string(file_b)
        .expect(&format!("Error reading file {}", file_b));

    (content_a, content_b)
}

pub fn write_patch_file(edits: &[DiffOp], filename: &str) {
    let mut file = File::create(filename).expect("An error occurred while creating the patch file.");

    for edit in edits.iter() {
        match edit {
            DiffOp::Match(line) => {
                println!("{}", line);
                writeln!(file, " {}", line).unwrap();
            }
            DiffOp::Insert(line) => {
                println!("{}", format!("+{}", line).green());
                writeln!(file, "+{}", line).unwrap();
            }
            DiffOp::Delete(line) => {
                println!("{}", format!("-{}", line).red());
                writeln!(file, "-{}", line).unwrap();
            }
        }
    }
}