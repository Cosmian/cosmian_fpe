use aes::Aes256;
use pyo3::{
    exceptions::{PyException, PyValueError},
    prelude::*,
    types::PyBytes,
};

use crate::ff1::{BinaryNumeralString, FF1 as FF1_Rust};

#[pyclass]
pub struct FF1(FF1_Rust<Aes256>);

#[pymethods]
impl FF1 {
    #[new]
    fn new(key: Vec<u8>, radix: u32) -> PyResult<Self> {
        match FF1_Rust::<Aes256>::new(&key, radix) {
            Ok(ff1) => Ok(Self(ff1)),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    pub fn encrypt(&self, x: Vec<u8>, py: Python) -> PyResult<Py<PyBytes>> {
        match self.0.encrypt(&[], &BinaryNumeralString::from_bytes_le(&x)) {
            Ok(ciphertext) => Ok(PyBytes::new(py, &ciphertext.to_bytes_le()).into()),
            Err(e) => Err(PyException::new_err(e.to_string())),
        }
    }

    pub fn decrypt(&self, x: Vec<u8>, py: Python) -> PyResult<Py<PyBytes>> {
        match self.0.decrypt(&[], &BinaryNumeralString::from_bytes_le(&x)) {
            Ok(plaintext) => Ok(PyBytes::new(py, &plaintext.to_bytes_le()).into()),
            Err(e) => Err(PyException::new_err(e.to_string())),
        }
    }
}
