use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bits<'i> {
    inner: &'i [u8],
    offset: u8,
}

impl<'i> Bits<'i> {
    pub fn new(slice: &'i [u8]) -> Bits<'i> {
        Bits {
            inner: slice,
            offset: 0,
        }
    }

    pub fn take(&mut self, bits: usize) -> Result<Vec<u8>, BitsError> {
        let (slice, offset, bytes) = grab_bits(self.inner, self.offset, bits)?;
        self.inner = slice;
        self.offset = offset;
        Ok(bytes)
    }

    pub fn peek(&self, bits: usize) -> Result<Vec<u8>, BitsError> {
        let (_, _, bytes) = grab_bits(self.inner, self.offset, bits)?;
        Ok(bytes)
    }

    // Length is returned as (bytes, offset), where the bytes is the
    // total length in bytes, and the offset is the number of bits offset
    // from that. The length in bits is therefore bytes * 8 - offset.
    pub fn len(&self) -> (usize, u8) {
        (self.inner.len(), self.offset)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum BitsError {
    #[error("{requested} bits were requested, but only {available} bits are available.")]
    InsufficientBits { requested: usize, available: usize },
}

fn byte_count(bits: usize) -> usize {
    let bytes = bits / 8;
    if bits % 8 > 0 {
        bytes + 1
    } else {
        bytes
    }
}

fn valid_request_size(slice: &[u8], offset: u8, request: usize) -> bool {
    debug_assert!(offset < 8);
    let req_byte_count = byte_count(request);
    let slice_overflow_bit_alignment = 8 - offset as usize;
    let req_overflow_bit_alignment = (request.wrapping_sub(1) % 8) + 1;
    req_byte_count < slice.len()
        || (req_byte_count == slice.len()
            && req_overflow_bit_alignment <= slice_overflow_bit_alignment)
}

fn grab_bits(
    mut slice: &[u8],
    mut offset: u8,
    mut bits: usize,
) -> Result<(&[u8], u8, Vec<u8>), BitsError> {
    debug_assert!(offset < 8);
    // Simple case
    if bits == 0 {
        return Ok((slice, offset, vec![]));
    }

    if !valid_request_size(slice, offset, bits) {
        return Err(BitsError::InsufficientBits {
            requested: bits,
            available: slice.len() * 8 + (offset.wrapping_sub(1) % 8) as usize - 7,
        });
    }

    // Pull chunks from the slice and add them to the container of bytes we are returning.
    // The size of the chunks varies depending on how the alignment of the request and the
    // slice compare.
    let mut bytes = Vec::with_capacity(byte_count(bits));
    let mut unaligned_bits = (bits % 8) as u8;
    while bits > 0 {
        if unaligned_bits > 0 {
            // We work with the smaller of the unaligned request bits, or the remaining bits
            // in the first byte of the slice.
            let chunk_size = std::cmp::min(8 - offset, unaligned_bits);
            let byte = (slice[0] << offset) >> (8 - chunk_size);
            offset += chunk_size;
            bits -= chunk_size as usize;
            unaligned_bits -= chunk_size;
            if offset >= 8 {
                slice = &slice[1..];
                offset -= 8;
            }
            match bytes.pop() {
                Some(value) => bytes.push((value << chunk_size) | byte),
                None => bytes.push(byte),
            }
        } else {
            let chunk_size = std::cmp::min(8 - offset as usize, bits) as u8;
            let byte = (slice[0] << offset) >> (8 - chunk_size);
            offset += chunk_size;
            bits -= chunk_size as usize;
            if offset >= 8 {
                slice = &slice[1..];
                offset -= 8;
            }
            if chunk_size < 8 && offset == 0 {
                unaligned_bits = 8 - chunk_size;
            }
            bytes.push(byte);
        }
    }
    Ok((slice, offset, bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_count() {
        assert_eq!(byte_count(0), 0);
        assert_eq!(byte_count(1), 1);
        assert_eq!(byte_count(8), 1);
        assert_eq!(byte_count(9), 2);
        assert_eq!(byte_count(16), 2);
        assert_eq!(byte_count(17), 3);
        assert_eq!(byte_count(24), 3);
        assert_eq!(byte_count(25), 4);
        assert_eq!(byte_count(usize::MAX), (usize::MAX / 8) + 1);
    }

    #[test]
    fn test_valid_request_size() {
        let slice = [1, 2, 3, 4, 5, 6, 7, 8];
        assert!(valid_request_size(&slice, 0, 0));
        assert!(valid_request_size(&slice, 7, 0));
        assert!(valid_request_size(&slice[7..], 7, 1));
        assert!(!valid_request_size(&slice[7..], 7, 2));
        assert!(valid_request_size(&slice[7..], 1, 7));
        assert!(!valid_request_size(&slice[7..], 1, 8));
        assert!(valid_request_size(&slice, 0, 16));
        assert!(valid_request_size(&slice, 0, 64));
        assert!(!valid_request_size(&slice, 0, 65));
    }

    #[test]
    fn basic_test() {
        let mut bits = Bits::new(&[0b00001111, 0b11000011]);
        assert_eq!(bits.take(4), Ok(vec![0b0]));
        assert_eq!(bits.peek(6), Ok(vec![0b111111]));
        assert_eq!(bits.peek(12), Ok(vec![0b00001111, 0b11000011]));
        assert_eq!(bits.take(4), Ok(vec![0b1111]));
        assert_eq!(bits.peek(2), Ok(vec![0b11]));
        assert_eq!(
            bits.peek(20),
            Err(BitsError::InsufficientBits {
                requested: 20,
                available: 8
            })
        );
        assert_eq!(bits.take(5), Ok(vec![0b00011000]));
        assert_eq!(bits.take(3), Ok(vec![0b00000011]))
    }
}
