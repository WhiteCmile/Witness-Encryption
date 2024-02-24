#![allow(non_snake_case)]
use crate::{Matrix, Range};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::adp::ADP;

pub enum FileType {
    READ,
    WRITE
}

pub struct FileOutput {
    pub file: File,
}

pub struct FileInput {
    reader: BufReader<File>,
}

impl FileInput {
    pub fn new(path: &str) -> FileInput {
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);
        FileInput {
            reader,
        }
    }

    fn read_numbers(&mut self) -> Vec<Range> {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed to read from file");
        input.split_whitespace()
            .into_iter()
            .map(|num| num.parse().expect("Input is not a number"))
            .collect()
    }

    pub fn input_matrix(&mut self) -> Matrix {
        let numbers = self.read_numbers();
        if numbers.len() != 2 {
            panic!("Invalid matrix size n, m");
        }
        let (n, m) = (numbers[0] as usize, numbers[1] as usize);
        let mut elements: Vec<Range> = Vec::with_capacity(n * m);
        for _ in 0..n {
            let numbers = self.read_numbers();
            if numbers.len() != m {
                panic!("Invalid matrix");
            }
            elements.extend(numbers.iter());
        }
        Matrix::from_vec(n, m, elements).transpose()
    }

    pub fn input_num(&mut self) -> Range {
        let numbers = self.read_numbers();
        if numbers.len() != 1 {
            panic!("Invalid input");
        }
        numbers[0]
    }

    pub fn input_adp(&mut self) -> ADP {
        let A = self.input_matrix();
        let k = A.nrows();
        let n = self.input_num() as usize;
        let mut B: Vec<Matrix> = Vec::with_capacity(n);
        for _ in 0..n {
            B.push(self.input_matrix());
        }
        ADP::new(n, k, A, B)
    }
}

impl FileOutput {

    pub fn new(path: &str) -> FileOutput {
        let file = File::create(path).expect("Failed to open file");
        FileOutput {
            file
        }
    }

    pub fn output_matrix(&mut self, mat: &Matrix) {
        let mut file = &self.file;
        let (n, m) = (mat.nrows(), mat.ncols());
        write!(file, "{} {}\n", n, m).expect("Failed to write to file");
        let matrix_string = matrix_to_string(mat);
        file.write_all(matrix_string.as_bytes()).expect("Failed to write to file");
    }

    pub fn output_num(&mut self, n: usize) {
        let output_str = n.to_string() + "\n";
        self.file.write_all(output_str.as_bytes()).expect("Failed to write to file");
    }

    pub fn drop(self) {
        drop(self.file);
    }
}

fn matrix_to_string(mat: &Matrix) -> String {
    let mut matrix_string = String::new();
    for i in 0..mat.nrows() {
        let line_string = mat
        .row(i)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
        matrix_string.push_str(&line_string);
        matrix_string.push_str("\n");
    }
    matrix_string
}