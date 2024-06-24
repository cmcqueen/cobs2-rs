//! This module contains functions for standard COBS encoding and decoding.

use crate::{Error, Result};

/// Calculate the minimum possible COBS encoded output size, for a given size of input data.
pub const fn encode_min_output_size(input_len: usize) -> usize {
    if input_len >= usize::max_value() - 1 {
        usize::max_value()
    } else {
        input_len + 1
    }
}

/// Calculate the maximum possible COBS encoded output size, for a given size of input data.
pub const fn encode_max_output_size(input_len: usize) -> usize {
    if input_len == 0 {
        1
    } else if input_len >= usize::max_value() - 253 {
        usize::max_value()
    } else {
        let increase = (input_len + 253) / 254;
        if input_len >= usize::max_value() - increase {
            usize::max_value()
        } else {
            input_len + increase
        }
    }
}

/// Common function for converting an iterator encoder's input iterator size hint to an output size hint.
fn encode_size_hint(in_hint: (usize, Option<usize>)) -> (usize, Option<usize>) {
    let lower_bound = encode_min_output_size(in_hint.0);
    let upper_bound = in_hint.1.map(|x| encode_max_output_size(x));
    (lower_bound, upper_bound)
}

/// Calculate the minimum possible decoded output size, for a given size of COBS-encoded input.
pub const fn decode_min_output_size(input_len: usize) -> usize {
    if input_len >= 1 {
        let increase = (input_len - 1) / 255;
        input_len - 1 - increase
    } else {
        0
    }
}

/// Calculate the maximum possible decoded output size, for a given size of COBS-encoded input.
pub const fn decode_max_output_size(input_len: usize) -> usize {
    if input_len > 1 {
        input_len - 1
    } else {
        0
    }
}

/// Common function for converting an iterator decoder's input iterator size hint to an output size hint.
fn decode_size_hint(in_hint: (usize, Option<usize>)) -> (usize, Option<usize>) {
    let lower_bound = decode_min_output_size(in_hint.0);
    let upper_bound = in_hint.1.map(|x| decode_max_output_size(x));
    (lower_bound, upper_bound)
}

/// Encode data into COBS encoded form, writing output to the given output buffer.
///
/// The output data is COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a reference to a suitably-sized output buffer.
/// [`encode_max_output_size()`] calculates the required output buffer size, for a given input
/// size.
///
/// The return value is a [`Result`] that in the [`Ok`] case is a slice of the valid data in the
/// output buffer.
///
/// The following errors could be returned:
///
/// * [`Error::OutputBufferTooSmall`]
///
/// Example:
///
///     let mut cobs_buf = [0x55_u8; 1000];
///     let data = b"ABC\0ghij\0xyz";
///     let data_cobs = cobs2::cobs::encode_array(&mut cobs_buf, data);
///     assert_eq!(data_cobs.unwrap(), b"\x04ABC\x05ghij\x04xyz");
///
pub fn encode_array<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> Result<&'a [u8]> {
    let mut code_i = 0;
    let mut out_i = 1;

    if code_i >= out_buf.len() {
        return Err(Error::OutputBufferTooSmall);
    }
    for x in in_buf {
        if out_i - code_i >= 0xFF {
            out_buf[code_i] = 0xFF;
            code_i = out_i;
            if code_i >= out_buf.len() {
                return Err(Error::OutputBufferTooSmall);
            }
            out_i = code_i + 1;
        }
        if *x == 0 {
            out_buf[code_i] = (out_i - code_i) as u8;
            code_i = out_i;
            if code_i >= out_buf.len() {
                return Err(Error::OutputBufferTooSmall);
            }
            out_i = code_i + 1;
        } else {
            if out_i >= out_buf.len() {
                return Err(Error::OutputBufferTooSmall);
            }
            out_buf[out_i] = *x;
            out_i += 1;
        }
    }

    // We've reached the end of the source data.
    // Finalise the remaining output. In particular, write the code (length) byte.
    // Update the pointer to calculate the final output length.
    out_buf[code_i] = (out_i - code_i) as u8;

    Ok(&out_buf[..out_i])
}

/// Encode data into COBS encoded form, returning output as a vector of `u8`.
///
/// The output data is COBS-encoded, containing no zero-bytes.
///
/// The return value is a [`Result`] that in the [`Ok`] case is a vector of `u8`.
///
///     let data = b"ABC\0ghij\0xyz";
///     let data_cobs = cobs2::cobs::encode_vector(data);
///     assert_eq!(data_cobs.unwrap(), b"\x04ABC\x05ghij\x04xyz");
///
#[cfg(feature = "alloc")]
pub fn encode_vector(in_buf: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    let mut code_i = 0;
    let mut run_len = 0_u8;
    let mut out_vec = alloc::vec::Vec::<u8>::with_capacity(encode_max_output_size(in_buf.len()));

    for x in in_buf {
        if run_len == 0xFF {
            out_vec[code_i] = 0xFF;
            code_i += 0xFF;
            run_len = 0;
        }
        if *x == 0 {
            if run_len == 0 {
                out_vec.push(1);
                code_i += 1;
            } else {
                out_vec[code_i] = run_len;
                code_i += run_len as usize;
            }
            run_len = 0;
        } else {
            if run_len == 0 {
                out_vec.push(0xFF);
                run_len = 1;
            }
            out_vec.push(*x);
            run_len += 1;
        }
    }

    // We've reached the end of the source data.
    // Finalise the remaining output. In particular, write the code (length) byte.
    if run_len == 0 {
        out_vec.push(1);
    } else {
        out_vec[code_i] = run_len;
    }

    Ok(out_vec)
}

struct EncodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    in_iter: I,
    eof: bool,
    last_run_0xff: bool,
    hold_write_i: u8,
    hold_read_i: u8,
    hold_buf: [u8; 255],
}

impl<I> EncodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    fn new(i: I) -> EncodeIterator<I> {
        return EncodeIterator {
            in_iter: i,
            eof: false,
            last_run_0xff: false,
            hold_write_i: 0,
            hold_read_i: 0,
            hold_buf: [1; 255],
        };
    }
}

impl<I> Iterator for EncodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hold_write_i != 0 {
            if self.hold_read_i < self.hold_write_i {
                let byte_val = self.hold_buf[self.hold_read_i as usize];
                self.hold_read_i += 1;
                return Some(byte_val);
            } else {
                self.hold_read_i = 0;
                self.hold_write_i = 0;
                // else drop through to loop below.
            }
        }
        if self.eof {
            return None;
        }
        loop {
            if self.hold_write_i == 0xFE {
                self.last_run_0xff = true;
                return Some(0xFF);
            } else {
                let in_iter_next = self.in_iter.next();
                let byte_val = in_iter_next.unwrap_or_else(|| {
                    self.eof = true;
                    0
                });
                if self.last_run_0xff {
                    self.last_run_0xff = false;
                    if self.eof {
                        return None;
                    }
                }
                if byte_val == 0 {
                    return Some(self.hold_write_i + 1);
                } else {
                    self.hold_buf[self.hold_write_i as usize] = byte_val;
                    self.hold_write_i += 1;
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let in_iter_size_hint = self.in_iter.size_hint();
        encode_size_hint(in_iter_size_hint)
    }
}

/// Encode data into COBS encoded form, getting data from a `u8` iterator, and providing the output as an iterator.
///
/// The output data is COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a `u8` iterator.
///
/// The return value is a `u8` iterator. This is suitable to [`Iterator::collect()`] into a byte
/// container.
///
///     let data = b"ABC\0ghij\0xyz".to_vec();
///     let data_cobs: Vec<u8> = cobs2::cobs::encode_iter(data.into_iter()).collect();
///     assert_eq!(data_cobs, b"\x04ABC\x05ghij\x04xyz");
///
pub fn encode_iter(i: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
    EncodeIterator::new(i)
}

/// Encode data into COBS encoded form, getting data from a `&u8` iterator, and providing the output as an iterator.
///
/// The output data is COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a `&u8` iterator.
///
/// The return value is a `u8` iterator. This is suitable to [`Iterator::collect()`] into a byte
/// container.
///
///     let data = b"ABC\0ghij\0xyz".to_vec();
///     let data_cobs: Vec<u8> = cobs2::cobs::encode_ref_iter(data.iter()).collect();
///     assert_eq!(data_cobs, b"\x04ABC\x05ghij\x04xyz");
///
pub fn encode_ref_iter<'a, I>(i: I) -> impl Iterator<Item = u8> + 'a
where
    I: Iterator<Item = &'a u8> + 'a,
{
    EncodeIterator::<_>::new(i.copied())
}

/// Decode COBS-encoded data, writing decoded data to the given output buffer.
///
/// The caller must provide a reference to a suitably-sized output buffer.
/// [`decode_max_output_size()`] calculates the required output buffer size, for a given input
/// size.
///
/// The return value is a [`Result`] that in the [`Ok`] case is a slice of the decoded data in the
/// output buffer.
///
/// The following errors could be returned:
///
/// * [`Error::OutputBufferTooSmall`]
/// * [`Error::ZeroInEncodedData`]
/// * [`Error::TruncatedEncodedData`]
///
/// Example:
///
///     let mut decode_buf = [0x55_u8; 1000];
///     let data_cobs = b"\x04ABC\x05ghij\x04xyz";
///     let decode_data = cobs2::cobs::decode_array(&mut decode_buf, data_cobs);
///     assert_eq!(decode_data.unwrap(), b"ABC\0ghij\0xyz");
///
pub fn decode_array<'a>(out_buf: &'a mut [u8], in_buf: &[u8]) -> Result<&'a [u8]> {
    let mut code_i = 0;
    let mut out_i = 0;

    if !in_buf.is_empty() {
        loop {
            let code = in_buf[code_i];
            if code == 0 {
                return Err(Error::ZeroInEncodedData);
            }
            for in_i in (code_i + 1)..(code_i + code as usize) {
                if in_i >= in_buf.len() {
                    return Err(Error::TruncatedEncodedData);
                }
                let in_byte = in_buf[in_i];
                if in_byte == 0 {
                    return Err(Error::ZeroInEncodedData);
                }
                if out_i >= out_buf.len() {
                    return Err(Error::OutputBufferTooSmall);
                }
                out_buf[out_i] = in_byte;
                out_i += 1;
            }
            code_i += code as usize;
            if code_i >= in_buf.len() {
                // End of data. Exit, without outputting a trailing zero for the end of the data.
                break;
            }
            if code < 0xFF {
                // Output trailing zero.
                if out_i >= out_buf.len() {
                    return Err(Error::OutputBufferTooSmall);
                }
                out_buf[out_i] = 0;
                out_i += 1;
            }
        }
    }
    Ok(&out_buf[..out_i])
}

/// Decode COBS-encoded data, returning output as a vector of `u8`.
///
/// The return value is a [`Result`] that in the [`Ok`] case is a vector of `u8`.
///
/// The following errors could be returned:
///
/// * [`Error::ZeroInEncodedData`]
/// * [`Error::TruncatedEncodedData`]
///
/// Example:
///
///     let data_cobs = b"\x04ABC\x05ghij\x04xyz";
///     let decode_data = cobs2::cobs::decode_vector(data_cobs);
///     assert_eq!(decode_data.unwrap(), b"ABC\0ghij\0xyz");
///
#[cfg(feature = "alloc")]
pub fn decode_vector(in_buf: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    let mut code_i = 0;
    let mut out_vec = alloc::vec::Vec::<u8>::with_capacity(decode_max_output_size(in_buf.len()));

    if !in_buf.is_empty() {
        loop {
            let code = in_buf[code_i];
            if code == 0 {
                return Err(Error::ZeroInEncodedData);
            }
            for in_i in (code_i + 1)..(code_i + code as usize) {
                if in_i >= in_buf.len() {
                    return Err(Error::TruncatedEncodedData);
                }
                let in_byte = in_buf[in_i];
                if in_byte == 0 {
                    return Err(Error::ZeroInEncodedData);
                }
                out_vec.push(in_byte);
            }
            code_i += code as usize;
            if code_i >= in_buf.len() {
                // End of data. Exit, without outputting a trailing zero for the end of the data.
                break;
            }
            if code < 0xFF {
                // Output trailing zero.
                out_vec.push(0);
            }
        }
    }
    Ok(out_vec)
}

struct DecodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    in_iter: I,
    eof: bool,
    last_run: u8,
    count_run: u8,
}

impl<I> DecodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    fn new(i: I) -> DecodeIterator<I> {
        return DecodeIterator {
            in_iter: i,
            eof: false,
            last_run: 0,
            count_run: 0,
        };
    }
}

impl<I> Iterator for DecodeIterator<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let in_iter_next = self.in_iter.next();
            let byte_val = in_iter_next.unwrap_or(0);
            if byte_val == 0 {
                return None;
            }
            if self.count_run == 0 {
                let last_run = self.last_run;
                self.last_run = byte_val;
                self.count_run = byte_val - 1;
                if last_run != 0 && last_run != 0xFF {
                    return Some(0);
                }
            } else {
                self.count_run -= 1;
                return Some(byte_val);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let in_iter_size_hint = self.in_iter.size_hint();
        decode_size_hint(in_iter_size_hint)
    }
}

/// Decode COBS-encoded data, getting data from a `u8` iterator, and providing the output as an iterator.
///
/// The input data should be COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a `u8` iterator.
///
/// The return value is a `u8` iterator. This is suitable to [`Iterator::collect()`] into a byte
/// container.
///
/// Unlike the other decode functions, no errors are returned by this function. Rather, decoding is
/// best-effort. In the event of any zero in the input, this will be regarded as end-of-data. If
/// insufficient bytes are present following a length code, the output will simply stop at the end
/// of the available data.
///
///     let data_cobs = b"\x04ABC\x05ghij\x04xyz".to_vec();
///     let decode_data: Vec<u8> = cobs2::cobs::decode_iter(data_cobs.into_iter()).collect();
///     assert_eq!(decode_data, b"ABC\0ghij\0xyz");
///
pub fn decode_iter<I>(i: I) -> impl Iterator<Item = u8>
where
    I: Iterator<Item = u8>,
{
    DecodeIterator::<I>::new(i)
}

/// Decode COBS-encoded data, getting data from a `&u8` iterator, and providing the output as an iterator.
///
/// The input data should be COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a `&u8` iterator.
///
/// The return value is a `u8` iterator. This is suitable to [`Iterator::collect()`] into a byte
/// container.
///
/// Unlike the other decode functions, no errors are returned by this function. Rather, decoding is
/// best-effort. In the event of any zero in the input, this will be regarded as end-of-data. If
/// insufficient bytes are present following a length code, the output will simply stop at the end
/// of the available data.
///
///     let data_cobs = b"\x04ABC\x05ghij\x04xyz".to_vec();
///     let decode_data: Vec<u8> = cobs2::cobs::decode_ref_iter(data_cobs.iter()).collect();
///     assert_eq!(decode_data, b"ABC\0ghij\0xyz");
///
pub fn decode_ref_iter<'a, I>(i: I) -> impl Iterator<Item = u8> + 'a
where
    I: Iterator<Item = &'a u8> + 'a,
{
    DecodeIterator::<_>::new(i.copied())
}

struct DecodeResultIterator<I>
where
    I: Iterator<Item = u8>,
{
    in_iter: I,
    eof: bool,
    last_run: u8,
    count_run: u8,
}

impl<I> DecodeResultIterator<I>
where
    I: Iterator<Item = u8>,
{
    fn new(i: I) -> DecodeResultIterator<I> {
        return DecodeResultIterator {
            in_iter: i,
            eof: false,
            last_run: 0,
            count_run: 0,
        };
    }
}

impl<I> Iterator for DecodeResultIterator<I>
where
    I: Iterator<Item = u8>,
{
    type Item = Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.eof {
                return None;
            }
            let in_iter_next = self.in_iter.next();
            if in_iter_next.is_none() {
                self.eof = true;
                if self.count_run != 0 {
                    return Some(Err(Error::TruncatedEncodedData));
                } else {
                    return None;
                }
            }
            let byte_val = in_iter_next.unwrap();
            if byte_val == 0 {
                self.eof = true;
                return Some(Err(Error::ZeroInEncodedData));
            }
            if self.count_run == 0 {
                let last_run = self.last_run;
                self.last_run = byte_val;
                self.count_run = byte_val - 1;
                if last_run != 0 && last_run != 0xFF {
                    return Some(Ok(0));
                }
            } else {
                self.count_run -= 1;
                return Some(Ok(byte_val));
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let in_iter_size_hint = self.in_iter.size_hint();
        decode_size_hint(in_iter_size_hint)
    }
}

/// Decode COBS-encoded data, getting data from a `u8` iterator, and providing the output as an iterator.
///
/// The input data should be COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a `u8` iterator.
///
/// The return value is a [`Result<u8>`] iterator. This is suitable to [`Iterator::collect()`] into a
/// byte container wrapped in [`Result`].
///
/// The following errors could be returned:
///
/// * [`Error::ZeroInEncodedData`]
/// * [`Error::TruncatedEncodedData`]
///
/// Example:
///
///     let data_cobs = b"\x04ABC\x05ghij\x04xyz".to_vec();
///     let decode_data: cobs2::Result<Vec<u8>> = cobs2::cobs::decode_result_iter(data_cobs.into_iter()).collect();
///     assert_eq!(decode_data.unwrap(), b"ABC\0ghij\0xyz");
///
pub fn decode_result_iter<I>(i: I) -> impl Iterator<Item = Result<u8>>
where
    I: Iterator<Item = u8>,
{
    DecodeResultIterator::<I>::new(i)
}

/// Decode COBS-encoded data, getting data from a `&u8` iterator, and providing the output as an iterator.
///
/// The input data should be COBS-encoded, containing no zero-bytes.
///
/// The caller must provide a `&u8` iterator.
///
/// The return value is a [`Result<u8>`] iterator. This is suitable to [`Iterator::collect()`] into a
/// byte container wrapped in [`Result`].
///
/// The following errors could be returned:
///
/// * [`Error::ZeroInEncodedData`]
/// * [`Error::TruncatedEncodedData`]
///
/// Example:
///
///     let data_cobs = b"\x04ABC\x05ghij\x04xyz".to_vec();
///     let decode_data: cobs2::Result<Vec<u8>> = cobs2::cobs::decode_result_ref_iter(data_cobs.iter()).collect();
///     assert_eq!(decode_data.unwrap(), b"ABC\0ghij\0xyz");
///
pub fn decode_result_ref_iter<'a, I>(i: I) -> impl Iterator<Item = Result<u8>> + 'a
where
    I: Iterator<Item = &'a u8> + 'a,
{
    DecodeResultIterator::<_>::new(i.copied())
}
