use std::{
    cmp::max,
    fs,
};

fn main() {
    let file_a = "src/fileA.txt";
    let file_b = "src/fileB.txt";

    let content_a = fs::read_to_string(file_a)
        .expect("Ocorreu um erro ao ler o FileA");

    let content_b = fs::read_to_string(file_b)
        .expect("Ocorreu um erro ao ler o FileB");

    let content_lines_a: Vec<&str> = content_a.lines().collect();
    let content_lines_b: Vec<&str> = content_b.lines().collect();

    // Remover prefixo e sufixo comuns
    let (mid_a, mid_b) = remove_comum_prefix_and_suffix(&content_lines_a, &content_lines_b);

    println!("Meio de A: {:?}", mid_a);
    println!("Meio de B: {:?}", mid_b);

    // Aplicar Myers diff sobre o “meio”
    myers_diff(mid_a, mid_b);
}

/// Remove prefixo e sufixo comuns
fn remove_comum_prefix_and_suffix<'a>(
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

fn myers_diff(content_a: &[&str], content_b: &[&str]) {
    let n = content_a.len();
    let m = content_b.len();

    let max = (n + m) as usize;
    let offset = max;

    let mut v: Vec<isize> = vec![0; 2 * max + 1];

    for d in 0..=max as isize {
        for k in (-d..=d).step_by(2) {
            let k_idx = (k + offset as isize) as usize;

            let mut x = if k == -d
                || (k != d && v[(k-1+offset as isize) as usize] < v[(k+1+offset as isize) as usize])
            {
                v[(k+1+offset as isize) as usize]
            } else {
                v[(k-1+offset as isize) as usize] + 1
            };

            let mut y = x - k;

            while (x as usize) < n && (y as usize) < m && content_a[x as usize] == content_b[y as usize] {
                x += 1;
                y += 1;
            }

            v[k_idx] = x;

            if (x as usize) >= n && (y as usize) >= m {
                return;
            }
        }
    }
}

