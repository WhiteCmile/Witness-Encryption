use my_lib::{config::Config, Matrix, Range};
use my_lib::adp::ADP;
use my_lib::random_gen;
use nalgebra::DMatrix;

pub fn gen_encoder(config: &Config, bit: u32, h: &Matrix, l: &Matrix) -> ADP {
    let Config { d, n, k, p} = *config;
    let mut R: Vec<Matrix> = Vec::with_capacity(d as usize);
    for _ in 0..d {
        R.push(random_gen::gen_matrix(k, k, p));
    }
    let mut A: Matrix = random_gen::gen_matrix(k, k, p);
    A *= bit as Range;
    for i in 0..d {
        // println!("{} {}", l[(0, i)], i);
        A -= l[(0, i)] * &R[i];
    }
    let mut B: Vec<Matrix> = Vec::with_capacity(n);
    for i in 0..n {
        let mut mat: Matrix = DMatrix::zeros(k, k);
        for j in 0..d {
            // println!("{} {}", h[(j, i)], j);
            mat += h[(j, i)] * &R[j];
        }
        B.push(mat);
    }
    ADP::new(n, k, A, B)
}