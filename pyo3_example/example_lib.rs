use pyo3::prelude::*;

// /// This module is implemented in Rust.
// #[pymodule]
// fn my_ext(_py: Python, m: &PyModule) -> PyResult<()> {
//     #[pyfn(m, "sum_as_string")]
//     fn sum_as_string_py(_py: Python, a: i64, b: i64) -> PyResult<String> {
//         let out = sum_as_string(a, b);
//         Ok(out)
//     }
//
//     Ok(())
// }

// logic implemented as a normal Rust function
// fn sum_as_string(a: i64, b: i64) -> String {
//     format!("{}", a + b)
// }

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
