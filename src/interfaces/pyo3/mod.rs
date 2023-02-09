//! Implement interfaces in Python.

use pyo3::prelude::*;

mod py_ff1;
use py_ff1::FF1;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn fpe(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<FF1>()?;
    Ok(())
}
