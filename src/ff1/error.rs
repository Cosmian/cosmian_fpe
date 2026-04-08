use core::fmt;

/// Error indicating that a radix was not in the supported range of values for FF1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidRadix(pub(super) u32);

impl fmt::Display for InvalidRadix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The radix {} is not in the range 2..=(1 << 16)", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidRadix {}

/// Error returned by [`FF1fr::new`](crate::ff1::FF1fr).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FF1NewError {
    /// The provided radix is not in the supported range.
    InvalidRadix(InvalidRadix),
    /// The key length does not match the chosen cipher's requirement.
    InvalidKeyLength,
}

impl From<InvalidRadix> for FF1NewError {
    fn from(e: InvalidRadix) -> Self {
        FF1NewError::InvalidRadix(e)
    }
}

impl fmt::Display for FF1NewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FF1NewError::InvalidRadix(e) => e.fmt(f),
            FF1NewError::InvalidKeyLength => {
                write!(f, "Invalid key length for the chosen cipher")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FF1NewError {}

/// Errors that can occur while using FF1 for encryption or decryption.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumeralStringError {
    /// The numeral string was not compatible with the configured radix.
    InvalidForRadix(u32),
    /// The numeral string was longer than the maximum allowed length for FF1.
    TooLong {
        /// The length of the numeral string.
        ns_len: usize,
        /// The maximum length allowed (in numerals) for a numeral string of its radix.
        max_len: usize,
    },
    /// The numeral string was shorter than the minimum allowed length for FF1.
    TooShort {
        /// The length of the numeral string.
        ns_len: usize,
        /// The minimum length allowed (in numerals) for a numeral string of its radix.
        min_len: usize,
    },
    /// The tweak was longer than `u32::MAX` bytes.
    TweakTooLong,
    /// A [`BinaryNumeralString`](crate::ff1::BinaryNumeralString) length is not a
    /// multiple of 8 and cannot be converted to bytes.
    NotByteAligned,
}

impl fmt::Display for NumeralStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumeralStringError::InvalidForRadix(radix) => {
                write!(f, "The given numeral string is invalid for radix {}", radix)
            }
            NumeralStringError::TooLong { ns_len, max_len } => write!(
                f,
                "The given numeral string is too long for FF1 ({} > {})",
                ns_len, max_len,
            ),
            NumeralStringError::TooShort { ns_len, min_len } => write!(
                f,
                "The given numeral string is too short for FF1 ({} < {})",
                ns_len, min_len,
            ),
            NumeralStringError::TweakTooLong => {
                write!(f, "The tweak is longer than u32::MAX bytes")
            }
            NumeralStringError::NotByteAligned => {
                write!(f, "BinaryNumeralString length is not a multiple of 8")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NumeralStringError {}
