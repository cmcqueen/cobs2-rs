//! Encoding and decoding of COBS (Consistent Overhead Byte Stuffing)
//!
//! ## Intro
//!
//! This crate provides functions for encoding and decoding of COBS, and a minor variant of COBS
//! known as COBS/R.
//!
//! ### What Is COBS?
//!
//! COBS is a method of encoding a packet of bytes into a form that contains no bytes with value
//! zero (`0x00`). The input packet of bytes can contain bytes in the full range of `0x00` to
//! `0xFF`. The COBS encoded packet is guaranteed to generate packets with bytes only in the range
//! `0x01` to `0xFF`. Thus, in a communication protocol, packet boundaries can be reliably
//! delimited with `0x00` bytes.
//!
//! The COBS encoding does have to increase the packet size to achieve this encoding. However,
//! compared to other byte-stuffing methods, the packet size increase is reasonable and
//! predictable. COBS always adds 1 byte to the message length. Additionally, for longer packets
//! of length *n*, it *may* add n/254 (rounded down) additional bytes to the encoded packet size.
//!
//! For example, compare to the PPP protocol, which uses `0x7E` bytes to delimit PPP packets. The
//! PPP protocol uses an "escape" style of byte stuffing, replacing all occurences of `0x7E` bytes
//! in the packet with `0x7D 0x5E`. But that byte-stuffing method can potentially double the size
//! of the packet in the worst case. COBS uses a different method for byte-stuffing, which has a
//! much more reasonable worst-case overhead.
//!
//! For more details about COBS, see the references.
//!
//! ### What is COBS/R?
//!
//! I have included a variant on COBS, COBS/R, which slightly modifies COBS to often avoid the +1
//! byte overhead of COBS. So in many cases, especially for smaller packets, the size of a COBS/R
//! encoded packet is the same size as the original packet. See the [cobsr] module for more details
//! about COBS/R.
//!
//! ### References
//!
//! Consistent Overhead Byte Stuffing  
//! Stuart Cheshire and Mary Baker  
//! IEEE/ACM Transations on Networking, Vol. 7, No. 2, April 1999
//!
//! Consistent Overhead Byte Stuffing (for IEEE)  
//! <http://www.stuartcheshire.org/papers/COBSforToN.pdf>
//!
//! PPP Consistent Overhead Byte Stuffing (COBS)  
//! PPP Working Group Internet Draft  
//! James Carlson, IronBridge Networks  
//! Stuart Cheshire and Mary Baker, Stanford University  
//! November 1997
//!
//! PPP Consistent Overhead Byte Stuffing (COBS)  
//! <http://tools.ietf.org/html/draft-ietf-pppext-cobs-00>
//!
//! ## License
//!
//! The code is released under the MIT license. See LICENSE.txt for details.

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
