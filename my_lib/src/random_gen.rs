use rand::Rng;
use crate::{Range, Matrix};

pub fn gen_matrix(n: usize, m: usize, p: Range) -> Matrix {
    let mut rng = rand::thread_rng();
    Matrix::from_fn(n, m, |_, _| rng.gen_range(0..p) as Range)
}

pub fn gen_col_vector(n: usize, p: Range) -> Matrix {
    gen_matrix(n, 1, p)
}

pub fn gen_row_vector(n: usize, p: Range) -> Matrix {
    gen_matrix(1, n, p)
}

pub fn gen_col_vectors(n: usize, k: usize, p: Range) -> Vec<Matrix> {
    let mut vecs: Vec<Matrix> = Vec::with_capacity(n);
    for _ in 0..n {
        vecs.push(gen_col_vector(k, p));
    }
    vecs
}

pub fn gen_row_vectors(n: usize, k: usize, p: Range) -> Vec<Matrix> {
    let mut vecs: Vec<Matrix> = Vec::with_capacity(n);
    for _ in 0..n {
        vecs.push(gen_row_vector(k, p));
    }
    vecs
}