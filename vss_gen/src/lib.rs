use ndarray::prelude::*;
use rand::Rng;
use my_lib::file_io::{self, FileIO};
use my_lib::stdio;
use my_lib::Matrix;

pub fn get_input() -> (usize, usize) {
    println!("Please input d and n of matrix H");
    let numbers = stdio::input_numbers::<usize>();

    if numbers.len() != 2 {
        panic!("Wrong number of input; expect two numbers");
    }

    (numbers[0], numbers[1])
}

pub fn gen_vector_sum(h: &Matrix, n: usize, m: usize) -> (Matrix, Matrix) {
    let mut rand_gen = rand::thread_rng();
    let mut col_indexes: Vec<usize> = Vec::new();

    for col in 0..m {
        if rand_gen.gen_bool(0.5) {
            col_indexes.push(col);
        }
    }

    let selected_vectors = h.select(Axis(1), &col_indexes.as_slice());
    let witness = Array::from_shape_vec((1, col_indexes.len()), 
        col_indexes
            .iter()
            .map(|num| *num as i32)
            .collect()).unwrap();
    let sum_vector = selected_vectors.sum_axis(Axis(1));

    let sum_vector: Matrix = Array::from_shape_vec((n, 1), sum_vector.to_vec()).unwrap();

    (witness, sum_vector)
}

pub fn gen_output(h: &Matrix, w: &Matrix, l: &Matrix) {
    println!("Write matrix H, l in vss.txt");
    let file_io = FileIO::new("vss.txt");
    file_io.output_matrix(h);
    file_io.output_matrix(l);
    println!("Do you want to output the witness? [Y/n]");
    let input = stdio::input_string();
    match input.as_str() {
        "y\n" | "Y\n" => { 
            println!("Write witness in witness.txt"); 
            let file_io = FileIO::new("witness.txt");
            file_io.output_matrix(w);
        },
        _ => (),
    }
}