use std::{
    cmp::max, fs::{self}
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

    mayers_diff(&content_lines_a, &content_lines_b);
}

fn mayers_diff(content_a: &[&str], content_b: &[&str]) {
    let vec = remove_comum_prefix_and_suffix(content_a, content_b);

    let len_a = vec.0.len();
    let len_b = vec.1.len();

    println!("new content a: {:?}", vec.0);
    println!("new content b: {:?}", vec.1);

    get_v(content_a, content_b, Vec::new(), 0, 0, 0, 0);
}

fn remove_comum_prefix_and_suffix<'a>(content_a: &'a [&'a str], content_b: &'a [&'a str]) -> (&'a [&'a str], &'a [&'a str]) {
    let mut prefix_len: usize = 0;
    let mut suffix_len: usize = 0;
    let len_a: usize = content_a.len();
    let len_b: usize = content_b.len();

    for (index, element_a) in content_a.iter().enumerate() {
        if index >= len_b || element_a != &content_b[index] {
            break;
        }
        prefix_len += 1;
    }

    while suffix_len < (len_a - prefix_len) 
        && suffix_len < (len_b - prefix_len) 
        && content_a[len_a - 1 - suffix_len] == content_b[len_b - 1 - suffix_len] {
        suffix_len += 1;
    }

    return (&content_a[prefix_len .. len_a-suffix_len], &content_b[prefix_len .. len_b-suffix_len]);
}

fn get_v(content_a: &[&str], content_b: &[&str], v:Vec<usize>, k: usize, d: usize, posX: usize, posY: usize) {
    let mut v: Vec<usize> = v;
    let mut k: usize = k;
    let d: usize = d;
    let mut posX: usize = posX;
    let mut posY: usize = posY;
    let mut x: usize = 0;

    k = posX - posY;
    let delecao = v.get(k - 1).unwrap_or(&0) + 1;
    let insercao = v.get(k + 1).unwrap_or(&0);
    x = max(delecao, *insercao);

    if content_a[posX] == content_b[posY] {
        // match
        v[k] = x;

        get_v(content_a, content_b, v, k, d, posX + 1, posY + 1);
    }
    else {
        // nao deu match -> edicao

        // edicao 
        if delecao > *insercao {
            get_v(content_a, content_b, v, k, d, posX + 1, posY);
        } else {
            get_v(content_a, content_b, v, k, d, posX, posY + 1);
        }
    }
}