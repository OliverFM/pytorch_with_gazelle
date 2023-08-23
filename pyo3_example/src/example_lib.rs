use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn example_lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

/*
// use pyo3::prelude::pyfunction;
// use pyo3::prelude::Python;
use pyo3::prelude::*;
// use pyo3::types::PyModule;
// use pyo3::PyResult;
//
/// This module is implemented in Rust.
#[pymodule]
fn ext(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "sum_as_string")]
    fn sum_as_string_py(_py: Python, a: i64, b: i64) -> PyResult<String> {
        let out = sum_as_string(a, b);
        Ok(out)
    }

    Ok(())
}

// logic implemented as a normal Rust function
fn sum_as_string(a: i64, b: i64) -> String {
    format!("{}", a + b)
}

// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }
// */
