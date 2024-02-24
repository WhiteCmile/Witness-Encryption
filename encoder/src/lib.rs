use my_lib::Matrix;
use std::env;
use my_lib::config::Config;
use my_lib::stdio;
use my_lib::file_io::FileInput;
use crate::all_acc_gen::{self, Noise};

pub fn get_opt() -> Box<dyn Noise> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid opt numbers");
    }
    let acc_gen = args.remove(1);
    let acc_gen: Box<dyn Noise> = match acc_gen.as_str() {
        "-form" => { Box::new(all_acc_gen::Form) },
        "-nss" => { Box::new(all_acc_gen::Nss) },
        _ => { panic!("Invalid"); }
    };
    acc_gen
}

pub fn get_input() -> (String, u32) {
    println!("Please input path of vss");
    let input = stdio::input_string();
    let path = input;
    println!("Please input the bit you want to encrypt");
    let numbers = stdio::input_numbers::<u32>();
    if numbers.len() != 1 {
        panic!("Invalid input number"); 
    }
    let bit = numbers[0];
    if bit != 0 && bit != 1 {
        panic!("The input bit should only be 1 or 0");
    }
    (path, bit)
}

pub fn get_config(path: &String) -> (Config, Matrix, Matrix) {
    let mut file_io = FileInput::new(path.as_str());
    let h = file_io.input_matrix();
    let l = file_io.input_matrix();
    let config = Config::new(&h);
    (config, h, l)
}