use ndarray::Array2;
use std::fs::{File, OpenOptions};
use std::io::Write;

type Matrix = Array2<i32>;

pub enum FileType {
    TRUNC,
    APPEND,
}

fn write_matrix(file: &mut File, mat: &Matrix) {
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

pub fn output_matrix(path: &str, mat: &Matrix, tp: FileType) {
    let mut file = match tp {
        FileType::TRUNC => { 
            match OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(path) {
                Ok(file) => file,
                Err(_) => {
                    File::create(path).expect("Failed to create file")
                },
            }
        },
        FileType::APPEND => OpenOptions::new()
                            .append(true)
                            .open(path)
                            .expect("Failed to open file"),
    };
    let (n, m) = mat.dim();
    write!(file, "{} {}\n", n, m).unwrap();
    write_matrix(&mut file, &mat);
}