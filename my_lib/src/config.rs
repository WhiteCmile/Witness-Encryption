use crate::Matrix;
use ndarray::Axis;

pub struct Config {
    pub d: usize,
    pub n: usize,
    pub k: usize,
    pub p: i32,
}

impl Config {
    fn calculate_k(n: usize) -> usize {
        n + 1
    }

    pub fn new(h: &Matrix) -> Config {
        let (d, n) = h.dim();
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

fn is_prime(n: i32) -> bool {
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

fn gen_prime(h: &Matrix) -> i32 {
    let sum_vector = h.sum_axis(Axis(1));
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