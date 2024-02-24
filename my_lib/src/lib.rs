use nalgebra::DMatrix;

pub mod file_io;
pub mod random_gen;
pub mod stdio;
pub mod config;
pub mod adp;

pub type Range = i64;
pub type Matrix = DMatrix<Range>;
