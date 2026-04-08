# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this library adheres to Rust's notion of
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- `fpe::ff1::{InvalidRadix, NumeralStringError}`
- `fpe::ff1::FF1NewError`: new error type returned by `FF1fr::new`, with
  variants `InvalidRadix` and `InvalidKeyLength`.
- `fpe::ff1::NumeralStringError::TweakTooLong`: returned by `encrypt`/`decrypt`
  when the tweak length exceeds `u32::MAX` bytes (NIST SP 800-38G §5.1).
- `fpe::ff1::NumeralStringError::NotByteAligned`: returned by
  `BinaryNumeralString::to_bytes_le` when the length is not a multiple of 8.

### Security
- `fpe::ff1`: `Prf` now implements `Drop` and zeroes its CBC output block
  buffer on drop, removing key-derived bytes from memory.
- `fpe::ff1::FF1fr`: the expanded cipher key schedule is zeroed on drop for
  any `CIPH` that implements `ZeroizeOnDrop` (all `aes 0.8` types do so by
  default).
- `fpe::ff1::FF1fr::new`: construction now panics if `FEISTEL_ROUNDS < 8`,
  preventing the creation of instances that would perform identity encryption.
- `fpe::ff1::FF1fr::new`: construction now panics if the cipher's block size
  is not 128 bits, enforcing the requirement of NIST SP 800-38G §4.3.

### Fixed
- `fpe::ff1::FF1fr::new`: now uses `KeyInit::new_from_slice` and returns
  `FF1NewError::InvalidKeyLength` instead of panicking on wrong-length keys.
- `fpe::ff1::alloc::BinaryNumeralString::to_bytes_le`: no longer panics on
  non-byte-aligned input; returns `Err(NumeralStringError::NotByteAligned)`.
- `fpe::ff1::alloc`: replaced O(n) manual exponentiation loop in `pow` with
  `num_traits::pow::pow`, which uses O(log n) binary exponentiation, closing a
  DoS vector for large numeral strings.
- `fpe::ff1::alloc`: `assert_eq!(radix, 2)` in `BinaryNumeralString::num_radix`
  and `str_radix` replaced with `debug_assert_eq!`; these are internal
  invariants always satisfied by the validated call path.
- `fpe::ff1::alloc`: removed redundant parentheses in `is_valid` closures for
  both `FlexibleNumeralString` and `BinaryNumeralString` (Clippy lint
  `clippy::unused_parens`).
- `fpe::ff1::test_vectors`: replaced deprecated `array::IntoIter::new([...])`
  with `IntoIterator::into_iter([...])` and removed the associated
  `#[allow(deprecated)]` attribute and unused `use core::array` import.

### Changed
- MSRV is now 1.70.0.
- Bumped dependencies to `cipher 0.4`, `cbc 0.1`; added `zeroize = "1"`.
  - `aes 0.8` is now the minimum compatible crate version.
- `fpe::ff1::FF1fr::new` now returns `Result<Self, FF1NewError>` instead of
  `Result<Self, InvalidRadix>`; `FF1NewError` also covers invalid key length.
- `fpe::ff1::encrypt` and `decrypt` now return
  `Err(NumeralStringError::TweakTooLong)` when the tweak exceeds `u32::MAX`
  bytes (previously the tweak length would silently truncate in the P-block).
- `fpe::ff1`: the 16-byte P-block construction is annotated with inline
  comments mapping each field to NIST SP 800-38G §6.2 Step 5. The `u` field
  now uses the explicit cast `(u % 256) as u8` instead of relying on implicit
  truncation.
- `fpe::ff1::Radix::to_u32`: duplicate match arms consolidated using an OR
  pattern (`Radix::Any { .. } | Radix::PowerTwo { .. } => radix`).
- `fpe::ff1`: documentation updated to reference the final NIST SP 800-38G
  (removed the draft Revision 1 URL).
- `fpe::ff1::FF1h`: documented as a non-standard extension that breaks CAVP
  compliance and interoperability with conforming FF1 implementations.
- `fpe::ff1`: module documentation notes that only FF1 is implemented; FF3
  (also defined in NIST SP 800-38G) is not provided.
- `fpe::ff1`:
  - `FF1::new` now returns `Result<_, InvalidRadix>`.
  - `FF1::{encrypt, decrypt}` now return `Result<_, NumeralStringError>`.
  - `FF1::{encrypt, decrypt}` now enforce minimum and maximum lengths for
    numeral strings. The minimum length depends on the radix, while the maximum
    length is `u32::MAX` numerals (which means `BinaryNumeralString` can process
    data that is less than 16 MiB).
  - Refactored `NumeralString` trait:
    - Renamed `NumeralString::len` to `NumeralString::numeral_count`.

## [0.5.1] - 2021-10-27
### Fixed
- Disabled the `num-bigint`, `num-integer`, and `num-traits` default features.
  These dependencies are behind the `alloc` feature flag, but some of their
  default features depend on `std`.

## [0.5.0] - 2021-07-31
### Added
- `no-std` support.
  - `libm` dependency for math operations that aren't available in `core`.
  - Functionality that requires allocations is now behind the (default-enabled)
    `alloc` feature flag.

### Changed
- MSRV is now 1.49.0.
- This crate now depends directly on the `cipher` crate for its traits instead
  of indirectly via the `aes` crate.
- Bumped dependencies to `cipher 0.3`, `block-modes 0.8`, `num-bigint 0.4`.
  - `aes 0.7` is now the minimum compatible crate version.
- `num-bigint`, `num-integer`, and `num-traits` dependencies are now behind the
  (default-enabled) `alloc` feature flag.

### Removed
- Direct dependency on the `aes` crate, enabling it to be dropped in contexts
  where an alternative AES implementation (or alternative compatible block
  cipher) is desired.

## [0.4.0] - 2021-01-27
### Changed
- MSRV is now 1.41.0.
- Bumped dependencies to `aes 0.6`, `block-modes 0.7`.

## [0.3.1] - 2020-08-16
### Changed
- `Numeral::from_bytes` now takes `impl Iterator<Item = u8>` instead of `&[u8]`.

### Fixed
- Subtraction overflow on empty input.

## [0.3.0] - 2020-08-15
### Added
- `Numeral` trait, representing the type used for numeric operations.
- `NumeralString::Num: Numeral` associated type.

### Changed
- MSRV is now 1.36.0.
- Bumped dependencies to `aes 0.5`, `block-modes 0.6`, `num-bigint 0.3`.
- `NumeralString::{num_radix, str_radix}` now use `u32` for the radix and
  `Self:Num` for the numeric form of the numeral string.

## [0.2.0] - 2019-07-22
### Changed
- MSRV is now 1.32.0.
- Bumped dependencies to `aes 0.2`.

## [0.1.0] - 2018-07-31
Initial release.
