# Consistent Overhead Byte Stuffing (COBS)

Rust functions for encoding and decoding COBS.

## Intro

The `cobs` package is provided, which contains modules containing functions
for encoding and decoding according to COBS methods.


### What Is COBS?

COBS is a method of encoding a packet of bytes into a form that contains no
bytes with value zero (`0x00`). The input packet of bytes can contain bytes
in the full range of `0x00` to `0xFF`. The COBS encoded packet is guaranteed to
generate packets with bytes only in the range `0x01` to `0xFF`. Thus, in a
communication protocol, packet boundaries can be reliably delimited with `0x00`
bytes.

The COBS encoding does have to increase the packet size to achieve this
encoding. However, compared to other byte-stuffing methods, the packet size
increase is reasonable and predictable. COBS always adds 1 byte to the
message length. Additionally, for longer packets of length *n*, it *may* add
n/254 (rounded down) additional bytes to the encoded packet size.

For example, compare to the PPP protocol, which uses `0x7E` bytes to delimit
PPP packets. The PPP protocol uses an "escape" style of byte stuffing,
replacing all occurences of `0x7E` bytes in the packet with `0x7D 0x5E`. But
that byte-stuffing method can potentially double the size of the packet in the
worst case. COBS uses a different method for byte-stuffing, which has a much
more reasonable worst-case overhead.

For more details about COBS, see the references.

I have included a variant on COBS, COBS/R, which slightly modifies COBS to
often avoid the +1 byte overhead of COBS. So in many cases, especially for
smaller packets, the size of a COBS/R encoded packet is the same size as the
original packet. See below for more details about COBS/R.

### References

Consistent Overhead Byte Stuffing  
Stuart Cheshire and Mary Baker  
IEEE/ACM Transations on Networking, Vol. 7, No. 2, April 1999

Consistent Overhead Byte Stuffing (for IEEE)  
http://www.stuartcheshire.org/papers/COBSforToN.pdf

PPP Consistent Overhead Byte Stuffing (COBS)  
PPP Working Group Internet Draft  
James Carlson, IronBridge Networks  
Stuart Cheshire and Mary Baker, Stanford University  
November 1997

PPP Consistent Overhead Byte Stuffing (COBS)  
http://tools.ietf.org/html/draft-ietf-pppext-cobs-00


## Modules Provided

* `cobs::cobs` — Consistent Overhead Byte Stuffing (basic method)
* `cobs::cobsr` — COBS/R — Consistent Overhead Byte Stuffing—Reduced

"Consistent Overhead Byte Stuffing—Reduced" (COBS/R) is my own invention,
a modification of basic COBS encoding, and is described in more detail below.

The following are not implemented:

* COBS/ZPE — Consistent Overhead Byte Stuffing—Zero Pair Elimination
* COBS/ZRE — Consistent Overhead Byte Stuffing—Zero Run Elimination

## Usage

The modules provide functions for encoding and decoding. Several implementations
are provided, which differ in the input and output data types.

* Arrays (no_std)
    * `encode_array()`
    * `decode_array()`
* Vectors
    * `encode_vector()`
    * `decode_vector()`

## Unit Testing

Unit testing is implemented:

    cargo test

## License

The code is released under the MIT license. See LICENSE.txt for details.

## Consistent Overhead Byte Stuffing—Reduced (COBS/R)

A modification of COBS, which I'm calling "Consistent Overhead Byte
Stuffing—Reduced" (COBS/R), is provided in the `cobs::cobsr` module. Its
purpose is to save one byte from the encoded form in some cases. Plain COBS
encoding always has a +1 byte encoding overhead. See the references for
details. COBS/R can often avoid the +1 byte, which can be a useful
savings if it is mostly small messages that are being encoded.

In plain COBS, the last length code byte in the message has some inherent
redundancy: if it is greater than the number of remaining bytes, this is
detected as an error.

In COBS/R, instead we opportunistically replace the final length code byte with
the final data byte, whenever the value of the final data byte is greater than
or equal to what the final length value would normally be. This variation can be
unambiguously decoded: the decoder notices that the length code is greater than
the number of remaining bytes.

### Examples

The byte values in the examples are in hex.

#### First example

Input:

    2F A2 00 92 73 02

This example is encoded the same in COBS and COBS/R. Encoded (length code bytes
are highlighted):

    **03** 2F A2 **04** 92 73 02

#### Second example

The second example is almost the same, except the final data byte value is
greater than what the length byte would be.

Input:

    2F A2 00 92 73 26

Encoded in plain COBS (length code bytes are highlighted):

    **03** 2F A2 **04** 92 73 26

Encoded in COBS/R:

    **03** 2F A2 **26** 92 73

Because the last data byte (`26`) is greater than the usual length code
(`04`), the last data byte can be inserted in place of the length code, and
removed from the end of the sequence. This avoids the usual +1 byte overhead of
the COBS encoding.

The decoder detects this variation on the encoding simply by detecting that the
length code is greater than the number of remaining bytes. That situation would
be a decoding error in regular COBS, but in COBS/R it is used to save one byte
in the encoded message.
