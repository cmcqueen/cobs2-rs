/*!
 * Encoding and decoding of COBS (Consistent Overhead Byte Stuffing)
 */

#![allow(dead_code)]
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

#[cfg(feature = "alloc")]
extern crate alloc;

/// Errors that can occur during COBS encoding/decoding.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    /// For functions that generate output in an array, such as [cobs::encode_array()], it
    /// indicates that the output array size is too small for the output data.
    OutputBufferTooSmall,
    /// For decoding functions, it indicates that an unexpected zero-byte was found in the
    /// input data. Valid COBS-encoded data should not contain any zero-bytes.
    /// This error is only applicable for decoding.
    ZeroInEncodedData,
    /// For COBS decoding functions, it indicates that the COBS-encoded data was not valid;
    /// the data appears to be truncated. Or it may be invalid due to data corruption.
    /// More data was expected given the last length-byte value in the data.
    /// This error is only applicable for COBS decoding (not COBS/R).
    TruncatedEncodedData,
}

/// Apply trait [std::error::Error].
#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Implement trait [fmt::Display].
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::OutputBufferTooSmall => {
                write!(f, "Output buffer is too small")
            }
            Error::ZeroInEncodedData => {
                write!(f, "Zero found in encoded input data")
            }
            Error::TruncatedEncodedData => {
                write!(f, "Unexpected end of encoded input data")
            }
        }
    }
}

/// The return type for encoding and decoding functions, based on [core::result::Result],
/// in which the error type is [Error].
pub type Result<T> = core::result::Result<T, crate::Error>;

pub mod cobs;

pub mod cobsr;
