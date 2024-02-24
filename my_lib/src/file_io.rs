use ndarray::Array2;
use crate::Matrix;
use std::fs::File;
use std::io::Write;

pub struct FileIO {
    pub file: File,
}

impl FileIO {
    pub fn new(path: &str) -> FileIO {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => 
                File::create(path).expect("Failed to open file"),
        };
        FileIO {
            file
        }
    }
    pub fn output_matrix(&self, mat: &Matrix) {
        let mut file = &self.file;
        let (n, m) = mat.dim();
        write!(file, "{} {}\n", n, m).unwrap();
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
}