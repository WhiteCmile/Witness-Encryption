use ndarray::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use crate::Matrix;

pub fn gen_matrix(n: usize, m: usize, p: i32) -> Matrix {
    Array::random((n, m), Uniform::new(0, p))
}

pub fn gen_col_vector(n: usize, p: i32) -> Matrix {
    gen_matrix(n, 1, p)
}

pub fn gen_row_vector(n: usize, p: i32) -> Matrix {
    gen_matrix(1, n, p)
}

pub fn gen_col_vectors(n: usize, d: usize, p: i32) -> Vec<Matrix> {
    let mut vecs: Vec<Matrix> = Vec::with_capacity(n);
    for _ in 0..n {
        vecs.push(gen_col_vector(d, p));
    }
    vecs
}

pub fn gen_row_vectors(n: usize, d: usize, p: i32) -> Vec<Matrix> {
    let mut vecs: Vec<Matrix> = Vec::with_capacity(n);
    for _ in 0..n {
        vecs.push(gen_row_vector(d, p));
    }
    vecs
}