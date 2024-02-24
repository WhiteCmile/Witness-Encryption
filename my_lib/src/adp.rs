#![allow(non_snake_case)]
use crate::Matrix;
use crate::file_io::FileOutput;

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
        assert_eq!((k, k), (A.nrows(), A.ncols()));
        ADP {
            n,
            k,
            A,
            B,
        }
    }
    pub fn output(&self, path: &str) {
        println!("Output write to {}", path);
        let mut file_io = FileOutput::new(path);
        file_io.output_matrix(&self.A);
        let n = self.B.len();
        file_io.output_num(n);
        for mat in self.B.iter() {
            file_io.output_matrix(mat);
        }
        file_io.drop();
    }
}

impl std::ops::Add for ADP {
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