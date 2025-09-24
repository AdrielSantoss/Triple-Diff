use std::fs::File;
use std::io::Write;

enum DiffOp<'a> {
    Match(&'a str),
    Insert(&'a str),
    Delete(&'a str),
}


pub fn remove_comum_prefix_and_suffix<'a>(
    content_a: &'a [&'a str], 
    content_b: &'a [&'a str]
) -> (&'a [&'a str], &'a [&'a str]) {
    let mut prefix_len = 0;
    let mut suffix_len = 0;
    let len_a = content_a.len();
    let len_b = content_b.len();

    for (i, &a) in content_a.iter().enumerate() {
        if i >= len_b || a != content_b[i] {
            break;
        }
        prefix_len += 1;
    }

    while suffix_len < (len_a - prefix_len) && suffix_len < (len_b - prefix_len) &&
          content_a[len_a - 1 - suffix_len] == content_b[len_b - 1 - suffix_len] {
        suffix_len += 1;
    }

    (&content_a[prefix_len..len_a-suffix_len], &content_b[prefix_len..len_b-suffix_len])
}

pub fn myers_diff(content_a: &[&str], content_b: &[&str]) {
    let n = content_a.len();
    let m = content_b.len();

    let max = (n + m) as usize;
    let offset = max;

    let mut v: Vec<isize> = vec![0; 2 * max + 1];
    let mut trace: Vec<Vec<isize>> = Vec::new();

    for d in 0..=max as isize {
        let mut y = 0;
        let mut x = 0;

        for k in (-d..=d).step_by(2) {
            let k_idx = (k + offset as isize) as usize;

            x = if k == -d || (k != d && v[(k-1+offset as isize) as usize] < v[(k+1+offset as isize) as usize])
            {
                v[(k+1+offset as isize) as usize]
            } else {
                v[(k-1+offset as isize) as usize] + 1
            };

            y = x - k;

            while (x as usize) < n && (y as usize) < m && content_a[x as usize] == content_b[y as usize] {
                x += 1;
                y += 1;
            }

            v[k_idx] = x;

            if (x as usize) >= n && (y as usize) >= m {
                println!("O tamanho da SES é: {}", d);
                trace.push(v.clone()); 
                break;
            }
        }

        if (x as usize) >= n && (y as usize) >= m {
            trace.push(v.clone()); 
            break;
        }

        trace.push(v.clone());
    }

    let mut x = n as isize;
    let mut y = m as isize;
    let mut edits: Vec<DiffOp> = Vec::new();

    for (d, v) in trace.iter().enumerate().rev() {
        let k = x - y;

        let d_isize = d as isize;

        let prev_k = if k == -d_isize
            || (k != d_isize && v[(k - 1 + offset as isize) as usize] < v[(k + 1 + offset as isize) as usize])
        {
            k + 1
        } else {
            k - 1
        };

        let prev_x = v[(prev_k + offset as isize) as usize];
        let prev_y = prev_x - prev_k;

        while x > prev_x && y > prev_y {
            edits.push(DiffOp::Match(content_a[(x-1) as usize]));

            x -= 1;
            y -= 1;
        }

        if d > 0 {
            if y > 0 && x == prev_x {
                edits.push(DiffOp::Insert(content_b[(y-1) as usize]));
                y -= 1;
            } else if x > 0 {
                edits.push(DiffOp::Delete(content_a[(x-1) as usize]));
                x -= 1;
            }
        }
    }

    write_patch_file(&edits, "patch.diff");
}

fn write_patch_file(edits: &[DiffOp], filename: &str) {
    let mut file = File::create(filename).expect("Não foi possível criar o arquivo de patch");

    for edit in edits.iter().rev() {
        match edit {
            DiffOp::Match(line) => {
                writeln!(file, " {}", line).unwrap();
            }
            DiffOp::Insert(line) => {
                writeln!(file, "+{}", line).unwrap();
            }
            DiffOp::Delete(line) => {
                writeln!(file, "-{}", line).unwrap();
            }
        }
    }
}

