use std::collections::HashMap;

use myers::myers_diff;
use utils::DiffOp;

pub fn patience_diff<'a>(content_a: &'a [&'a str], content_b: &'a [&'a str]) -> Vec<DiffOp<'a>> {
    let anchors = find_unique_anchors(content_a, content_b);

    if anchors.is_empty() {
        return myers_diff(content_a, content_b);
    }

    let positions_b: Vec<usize> = anchors.iter().map(|(_, _, idx_b)| *idx_b).collect();
    let lis_idx = get_lis_indices(&positions_b);
    let anchors_final: Vec<_> = lis_idx.iter().map(|&i| anchors[i]).collect();

    let mut diff = Vec::new();
    let mut last_a = 0;
    let mut last_b = 0;

    for &(anchor_line, idx_a, idx_b) in &anchors_final {
        let sub_a = &content_a[last_a..idx_a];
        let sub_b = &content_b[last_b..idx_b];

        if !sub_a.is_empty() || !sub_b.is_empty() {
            diff.extend(myers_diff(sub_a, sub_b));
        }

        diff.push(DiffOp::Match(anchor_line));

        last_a = idx_a + 1;
        last_b = idx_b + 1;
    }

    let sub_a = &content_a[last_a..];
    let sub_b = &content_b[last_b..];
    if !sub_a.is_empty() || !sub_b.is_empty() {
        diff.extend(myers_diff(sub_a, sub_b));
    }

    diff
}

fn find_unique_anchors<'a>(content_a: &'a [&'a str], content_b: &'a [&'a str]) -> Vec<(&'a str, usize, usize)> {
    let unique_a = get_unique_lines(content_a);
    let unique_b = get_unique_lines(content_b);

    let mut anchors = Vec::new();
    for &line in content_a.iter() {
        if let (Some(&idx_a), Some(&idx_b)) = (unique_a.get(line), unique_b.get(line)) {
            anchors.push((line, idx_a, idx_b));
        }
    }

    anchors
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

    result
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

    lis
}