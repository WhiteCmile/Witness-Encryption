use my_lib::{config::Config, Matrix};
use my_lib::adp::ADP;
use my_lib::random_gen;
use ndarray::Array2;

pub fn gen_encoder(config: &Config, bit: u32, h: &Matrix, l: &Matrix) -> ADP {
    let Config { d, n, k, p} = *config;
    let mut R: Vec<Matrix> = Vec::with_capacity(d as usize);
    for _ in 0..d {
        R.push(random_gen::gen_matrix(k, k, p));
    }
    let mut A: Matrix = random_gen::gen_matrix(k, k, p);
    A = bit as i32 * &A;
    for i in 0..d {
        A = A - l[[i, 0]] * &R[i];
    }
    let mut B: Vec<Matrix> = Vec::with_capacity(n);
    for i in 0..n {
        let mut mat: Matrix = Array2::zeros((k, k));
        for j in 0..d {
            mat = mat + h[[j, i]] * &R[j];
        }
        B.push(mat);
    }
    ADP::new(n, k, A, B)
}