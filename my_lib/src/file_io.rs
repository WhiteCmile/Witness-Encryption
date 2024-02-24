use ndarray::prelude::*;
use crate::Matrix;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

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

    fn read_numbers(&mut self) -> Vec<i32> {
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
        let mut elements: Vec<i32> = Vec::with_capacity(n * m);
        for _ in 0..n {
            let numbers = self.read_numbers();
            if numbers.len() != m {
                panic!("Invalid matrix");
            }
            elements.extend(numbers.iter());
        }
        Array::from_shape_vec((n, m), elements).expect("Failed to construct matrix")
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
        let (n, m) = mat.dim();
        write!(file, "{} {}\n", n, m).expect("Failed to write to file");
        let matrix_string = mat
            .rows()
            .into_iter()
            .map(|line|
                line
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(" "))
            .collect::<Vec<String>>()
            .join("\n") + "\n";
        file.write_all(matrix_string.as_bytes()).expect("Failed to write to file");
    }

    pub fn drop(self) {
        drop(self.file);
    }
}