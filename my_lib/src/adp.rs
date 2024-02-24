use std::ops::Add;
use ndarray::prelude::*;
type Matrix = Array2<i32>;

#[derive(Debug)]
pub struct ADP {
    pub n: usize,
    pub k: usize,
    pub A: Matrix,
    pub B: Vec<Matrix>,
}

impl ADP {
    pub fn new(n: usize, k: usize, A: Matrix, B: Vec<Matrix>) -> ADP {
        assert_eq!(B.len(), n);
        assert_eq!((k, k), A.dim());
        ADP {
            n,
            k,
            A,
            B,
        }
    }
}

impl Add for ADP {
    type Output = ADP;
    fn add(self, other: ADP) -> ADP {
        if self.n != other.n || self.k != other.k {
            panic!("Invalid ADP add");
        }
        let mut new_b: Vec<Matrix> = Vec::with_capacity(self.n);
        for i in 0..self.n {
            new_b.push(&self.B[i] + &other.B[i]);
        }
        ADP {
            n: self.n,
            k: self.k,
            A: self.A + other.A,
            B: new_b,
        }
    }
}