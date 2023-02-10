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

    /// Format preserving encryption of binary numeral string
    ///
    /// Args:
    /// plaintext (bytes): data to encrypt
    ///
    /// Returns:
    ///     bytes: ciphertext
    pub fn encrypt(&self, plaintext: Vec<u8>, py: Python) -> PyResult<Py<PyBytes>> {
        match self
            .0
            .encrypt(&[], &BinaryNumeralString::from_bytes_le(&plaintext))
        {
            Ok(ciphertext) => Ok(PyBytes::new(py, &ciphertext.to_bytes_le()).into()),
            Err(e) => Err(PyException::new_err(e.to_string())),
        }
    }

    /// Format preserving decryption of binary numeral string
    ///
    /// Args:
    ///     ciphertext (bytes): data to decrypt
    ///
    /// Returns:
    ///     bytes: plaintext
    pub fn decrypt(&self, ciphertext: Vec<u8>, py: Python) -> PyResult<Py<PyBytes>> {
        match self
            .0
            .decrypt(&[], &BinaryNumeralString::from_bytes_le(&ciphertext))
        {
            Ok(plaintext) => Ok(PyBytes::new(py, &plaintext.to_bytes_le()).into()),
            Err(e) => Err(PyException::new_err(e.to_string())),
        }
    }
}
