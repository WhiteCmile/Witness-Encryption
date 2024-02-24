use ndarray::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
type Matrix = Array2<i32>;

pub fn gen_matrix(n: usize, m: usize, p: i32) -> Array2<i32> {
    Array::random((n, m), Uniform::new(0, p))
}

pub fn gen_col_vector(n: usize, p: i32) -> Array2<i32> {
    gen_matrix(n, 1, p)
}

pub fn gen_row_vector(n: usize, p: i32) -> Array2<i32> {
    gen_matrix(1, n, p)
}

pub fn gen_col_vectors(n: usize, d: usize, p: i32) -> Vec<Array2<i32>> {
    let mut vecs: Vec<Array2<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        vecs.push(gen_col_vector(d, p));
    }
    vecs
}

pub fn gen_row_vectors(n: usize, d: usize, p: i32) -> Vec<Array2<i32>> {
    let mut vecs: Vec<Array2<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        vecs.push(gen_row_vector(d, p));
    }
    vecs
}