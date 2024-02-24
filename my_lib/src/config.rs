use crate::{Matrix, Range};
use nalgebra::{DMatrix, DVector};

pub struct Config {
    pub d: usize,
    pub n: usize,
    pub k: usize,
    pub p: Range,
}

impl Config {
    fn calculate_k(n: usize) -> usize {
        n + 1
    }

    pub fn new(h: &Matrix) -> Config {
        let (d, n) = (h.nrows(), h.ncols());
        let k = Config::calculate_k(n);
        let p = gen_prime(h);
        Config {
            d,
            n,
            k,
            p,
        }
    }
}

fn is_prime(n: Range) -> bool {
    for i in 2..n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn gen_prime(h: &Matrix) -> Range {
    let sum_vector = col_sum(h);
    let max_elem = sum_vector.iter().max();
    match max_elem {
        Some(max_elem) => {
            let mut max_elem = *max_elem;
            loop {
                if is_prime(max_elem) {
                    return max_elem;
                }
                max_elem += 1;
            }
        },
        _ => { panic!("Error"); }
    }
}

fn col_sum(mat: &Matrix) -> Matrix {
    let mut sum_vector = DVector::zeros(mat.nrows());
    for i in 0..mat.ncols() {
        sum_vector += mat.column(i);
    }
    let res_vector = DMatrix::from_columns(&[sum_vector]);
    res_vector
}