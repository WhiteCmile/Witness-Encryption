use my_lib::adp::ADP;
use my_lib::random_gen;
use my_lib::config::Config;
use my_lib::Matrix;
use nalgebra::DMatrix;

pub trait Noise {
    fn gen_noise(&self, config: &Config) -> ADP;
}

pub struct Form;
pub struct Nss;

impl Noise for Form {
    fn gen_noise(&self, config: &Config) -> ADP {
        let Config { n , k, p, ..} = *config;
        let u = random_gen::gen_col_vectors(n, k, p);
        let v = random_gen::gen_row_vectors(n, k, p);
        let s = random_gen::gen_col_vectors(n, k, p);
        let t = random_gen::gen_row_vectors(n, k, p);
        let c = random_gen::gen_matrix(n, n, p);
        let mut uv: Vec<Matrix> = Vec::with_capacity(n);
        let mut st: Vec<Matrix> = Vec::with_capacity(n);
        let mut ut: Vec<Matrix> = Vec::with_capacity(n);
        for i in 0..n {
            uv.push(&u[i] * &v[i]);
            st.push(&s[i] * &t[i]);
            ut.push(&u[i] * &t[i]);
        }
        let mut A = DMatrix::zeros(k, k);
        for i in 0..n {
            A += &uv[i];
        }
        let mut B: Vec<Matrix> = Vec::with_capacity(n);
        for i in 0..n {
            B.push(-&uv[i] + &st[i]);
            for j in 0..n {
                B[i] += c[(i, j)] * &ut[j];
            }
        }
        ADP::new(n, k, A, B)
    }
}

impl Noise for Nss {
    fn gen_noise(&self, config: &Config) -> ADP {
        let Config { n, k, .. } = *config;
        let A: Matrix = DMatrix::zeros(k, k);
        let B: Vec<Matrix> = Vec::with_capacity(n);
        ADP::new(n, k, A, B)
    }
}