use myers::{myers_diff};
use utils::{write_patch_file, get_content_files};

fn main() {
    let (content_a, content_b) = get_content_files();
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