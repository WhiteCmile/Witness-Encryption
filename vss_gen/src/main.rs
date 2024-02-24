mod lib;
use my_lib::{self, random_gen};
use my_lib::Matrix;

fn main() {
    let (d, n) = lib::get_input();
    let range = 50;
    let h: Matrix = random_gen::gen_matrix(d, n, range);
    let (witness, l) = lib::gen_vector_sum(&h, d, n);
    lib::gen_output(&h, &witness, &l);
}
