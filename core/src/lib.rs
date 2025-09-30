pub mod ffi;

pub fn process_matrix(input: &[f64]) -> Vec<f64> {
    input.iter().map(|&value| value * 2.0).collect()
}
