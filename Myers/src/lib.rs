use utils::{DiffOp};

pub fn remove_comum_prefix_and_suffix<'a>(
    a: &'a [&'a str],
    b: &'a [&'a str],
) -> (&'a [&'a str], &'a [&'a str], &'a [&'a str], &'a [&'a str]) {
    let mut start = 0;
    let mut end_a = a.len();
    let mut end_b = b.len();

    while start < end_a && start < end_b && a[start] == b[start] {
        start += 1;
    }

    while end_a > start && end_b > start && a[end_a - 1] == b[end_b - 1] {
        end_a -= 1;
        end_b -= 1;
    }

    let prefix = &a[..start];
    let mid_a = &a[start..end_a];
    let mid_b = &b[start..end_b];
    let suffix = &a[end_a..];

    return (prefix, mid_a, mid_b, suffix)
}

pub fn myers_diff<'a>(content_a: &'a [&'a str], content_b: &'a [&'a str]) -> Vec<DiffOp<'a>> {
    let (prefix, mid_a, mid_b, suffix_a) = remove_comum_prefix_and_suffix(content_a, content_b);

    let n = mid_a.len();
    let m = mid_b.len();
    let max = (n + m) as usize;
    let offset = max;

    let trace = forward(max, offset, n, m, mid_a, mid_b);
    let mut edits: Vec<DiffOp<'a>> = Vec::new();
    
    for line in prefix {
        edits.push(DiffOp::Match(line));
    }

    edits.extend(backtracking(trace, offset, n, m, mid_a, mid_b));

    for line in suffix_a {
        edits.push(DiffOp::Match(line));
    }
    
    edits
}

// Short Edit Script (SES)
fn forward(
    max: usize,
    offset: usize,
    n: usize,
    m: usize,
    content_a: &[&str],
    content_b: &[&str]
) -> Vec<(isize, Vec<isize>)> {
    let mut v: Vec<isize> = vec![0; 2 * max + 1];
    let mut trace: Vec<(isize, Vec<isize>)> = Vec::new();

    for d in 0..=max as isize {
        trace.push((d, v.clone()));

        for k in (-d..=d).step_by(2) {
            let k_idx = (k + offset as isize) as usize;

            let mut x = if k == -d || (k != d && v[(k-1+offset as isize) as usize] < v[(k+1+offset as isize) as usize])
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
                break;
            }
        }
    }

    trace
}

// Traceback
fn backtracking<'a>(
    trace: Vec<(isize, Vec<isize>)>,
    offset: usize,
    n: usize,
    m: usize,
    content_a: &'a [&'a str],
    content_b: &'a [&'a str],
) -> Vec<DiffOp<'a>> {
    let mut edits: Vec<DiffOp<'a>> = Vec::new();
    let mut x = n as isize;
    let mut y = m as isize;

    for (d, v) in trace.iter().rev() {
        let d = *d as isize;
        let k = x - y;

        let prev_k = if k == -d
            || (k != d && v[(k - 1 + offset as isize) as usize] < v[(k + 1 + offset as isize) as usize])
        {
            k + 1
        } else {
            k - 1
        };

        let prev_x = v[(prev_k + offset as isize) as usize];
        let prev_y = prev_x - prev_k;

        while x > prev_x && y > prev_y {
            edits.insert(0, DiffOp::Match(content_a[(x - 1) as usize]));   
            x -= 1;
            y -= 1;
        }

        if x == prev_x && y > prev_y {
            if y > 0 {
                edits.insert(0, DiffOp::Insert(content_b[(y - 1) as usize]));
            }
            y -= 1;
        }
        else if x > prev_x && y == prev_y {
            if x > 0 {
                edits.insert(0, DiffOp::Delete(content_a[(x - 1) as usize]));
            }
            x -= 1;
        }
    }

    edits
}