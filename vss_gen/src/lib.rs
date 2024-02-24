use rand::Rng;
use nalgebra::{DMatrix, DVector};
use my_lib::file_io::{FileOutput};
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
    let mut witness = vec![0; m];
    let mut sum_vector = DVector::zeros(n);

    for i in 0..m {
        if rand_gen.gen_bool(0.5) {
            let col = h.column(i);
            sum_vector += h.column(i);
            witness[i] = 1;
        }
    }

    let res_vector = DMatrix::from_columns(&[sum_vector]);

    let witness = DMatrix::from_vec(1, m, witness);

    (witness, res_vector)
}

pub fn gen_output(h: &Matrix, w: &Matrix, l: &Matrix) {
    println!("Write matrix H, l in vss.txt");
    let mut file_io = FileOutput::new("vss.txt");
    file_io.output_matrix(h);
    file_io.output_matrix(l);
    file_io.drop();
    println!("Do you want to output the witness? [Y/n]");
    let input = stdio::input_string();
    match input.as_str() {
        "y" | "Y" => { 
            println!("Write witness in witness.txt"); 
            let mut file_io = FileOutput::new("witness.txt");
            file_io.output_matrix(w);
            file_io.drop();
        },
        _ => (),
    }
}