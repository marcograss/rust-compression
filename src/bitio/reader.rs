//! rust-compression
//!
//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! <http://mozilla.org/MPL/2.0/>.

#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use bitio::direction::Direction;
use bitio::small_bit_vec::SmallBitVec;
use cbuffer::CircularBuffer;
use core::cmp;
use core::iter::Iterator;
use core::marker::PhantomData;
use core::mem::size_of;
use core::ops::{BitOrAssign, Shl, Shr};
use num_traits::sign::Unsigned;

pub trait BitRead<D: Direction> {
    type Iter;

    fn peek_bits<T: Unsigned>(
        &mut self,
        len: usize,
    ) -> Result<SmallBitVec<T>, String>
    where
        T: BitOrAssign
            + Shl<usize, Output = T>
            + Shr<usize, Output = T>
            + From<u8>;

    fn skip_bits(&mut self, len: usize) -> Result<usize, String>;
    fn read_bits<T: Unsigned>(
        &mut self,
        len: usize,
    ) -> Result<SmallBitVec<T>, String>
    where
        T: BitOrAssign
            + Shl<usize, Output = T>
            + Shr<usize, Output = T>
            + From<u8>;

    fn skip_to_next_byte(&mut self) -> usize;
}

#[derive(Clone)]
pub struct BitReader<D: Direction, R: Iterator> {
    inner: R,
    buf: u8,
    counter: usize,
    cbuf: CircularBuffer<u8>,
    pos: usize,
    phantom: PhantomData<fn() -> D>,
}

impl<D: Direction, R: Iterator<Item = u8>> BitRead<D> for BitReader<D, R> {
    type Iter = R;

    fn peek_bits<T: Unsigned>(
        &mut self,
        len: usize,
    ) -> Result<SmallBitVec<T>, String>
    where
        T: BitOrAssign
            + Shl<usize, Output = T>
            + Shr<usize, Output = T>
            + From<u8>,
    {
        let firstlen = cmp::min(len, self.counter);
        let needlen = (len - firstlen + 7) >> 3;

        if needlen > 0 {
            // バッファに読み込む
            if needlen > self.pos {
                if needlen + self.pos > self.buffer_cap() {
                    return Err("len is too long".to_owned());
                }
                let rbuf = (&mut self.inner)
                    .take(needlen - self.pos)
                    .collect::<Vec<u8>>();
                self.cbuf.append(&rbuf);
                self.pos += rbuf.len();
            }
            let mut ret = Self::conv_u8_to_t(self.buf);
            let mut count = self.counter;
            for i in (0..cmp::min(self.pos, needlen))
                .map(|x| Self::conv_u8_to_t(self.cbuf[self.pos - x - 1]))
            {
                ret |= D::backward(i, count);
                count += size_of::<u8>() << 3;
            }
            let retlen = cmp::min(count, len);
            Ok(if retlen != 0 {
                SmallBitVec::new(
                    D::convert(ret, size_of::<T>() << 3, retlen),
                    retlen,
                )
            } else {
                SmallBitVec::new(T::zero(), 0)
            })
        } else {
            Ok(SmallBitVec::new(
                D::convert(
                    T::from(self.buf),
                    size_of::<u8>() << 3,
                    firstlen,
                ),
                firstlen,
            ))
        }
    }

    fn skip_bits(&mut self, len: usize) -> Result<usize, String> {
        let firstlen = cmp::min(len, self.counter);
        let midlen = (len - firstlen) >> 3;
        let mut lastlen = (len - firstlen) & 0x07;

        if lastlen > 0 || midlen > 0 {
            // バッファに読み込む
            let needlen = midlen + if lastlen > 0 { 1 } else { 0 };
            if needlen > self.pos {
                if needlen + self.pos > self.buffer_cap() {
                    return Err("len is too long".to_owned());
                }
                let rbuf = (&mut self.inner)
                    .take(needlen - self.pos)
                    .collect::<Vec<u8>>();
                self.cbuf.append(&rbuf);
                self.pos += rbuf.len();
            }
            if midlen == self.pos {
                lastlen = 0;
            }
            let readlen = cmp::min(self.pos, needlen);
            self.pos -= readlen;
            if lastlen > 0 {
                self.buf = D::forward(self.cbuf[self.pos], lastlen);
                self.counter = (size_of::<u8>() << 3) - lastlen;
            } else {
                self.buf = 0;
                self.counter = 0;
            }
            Ok(firstlen + cmp::min((midlen << 3) + lastlen, readlen << 3))
        } else {
            self.buf = D::forward(self.buf, firstlen);
            self.counter -= firstlen;
            Ok(firstlen)
        }
    }

    fn read_bits<T: Unsigned>(
        &mut self,
        len: usize,
    ) -> Result<SmallBitVec<T>, String>
    where
        T: BitOrAssign
            + Shl<usize, Output = T>
            + Shr<usize, Output = T>
            + From<u8>,
    {
        let r = self.peek_bits::<T>(len);
        if let Ok(ref l) = r {
            try!(self.skip_bits(l.len()));
        }
        r
    }

    fn skip_to_next_byte(&mut self) -> usize {
        let len = self.counter;
        self.buf = 0;
        self.counter = 0;
        len
    }
}

const DEFAULT_BUF_SIZE: usize = 8; // u64まで対応可能

impl<D: Direction, R: Iterator<Item = u8>> BitReader<D, R> {
    #[inline]
    pub fn new<I>(inner: I) -> Self
    where
        I: IntoIterator<IntoIter = R, Item = u8>,
    {
        Self::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    #[inline]
    pub fn with_capacity<I>(cap: usize, inner: I) -> Self
    where
        I: IntoIterator<IntoIter = R, Item = u8>,
    {
        Self {
            inner: inner.into_iter(),
            buf: 0,
            counter: 0,
            cbuf: CircularBuffer::<u8>::new(cap),
            pos: 0,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn buffer_cap(&self) -> usize {
        self.cbuf.cap() - self.pos
    }

    #[inline]
    fn conv_u8_to_t<T: Unsigned>(value: u8) -> T
    where
        T: Shl<usize, Output = T> + Shr<usize, Output = T> + From<u8>,
    {
        D::convert(
            T::from(value),
            size_of::<u8>() << 3,
            size_of::<T>() << 3,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use action::Action;
    use bitio::direction::left::Left;
    use bitio::direction::right::Right;
    use bitio::writer::{BitWriteExt, BitWriter};

    #[test]
    fn leftbitreader_read() {
        let cursor = vec![0b1100_1100];

        let mut reader = BitReader::<Left, _>::new(cursor);

        assert_eq!(
            reader.read_bits::<u32>(1).ok(),
            Some(SmallBitVec::new(0b1, 1))
        );
        assert_eq!(
            reader.read_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(0b10, 2))
        );
        assert_eq!(
            reader.read_bits::<u32>(3).ok(),
            Some(SmallBitVec::new(0b011, 3))
        );
        assert_eq!(
            reader.read_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(0b00, 2))
        );
    }

    #[test]
    fn leftbitreader_readmulti() {
        let cursor = vec![243, 221, 190, 200];

        let mut reader = BitReader::<Left, _>::new(cursor);

        assert_eq!(
            reader.read_bits::<u32>(10).ok(),
            Some(SmallBitVec::new(975, 10))
        );
        assert_eq!(
            reader.read_bits::<u32>(10).ok(),
            Some(SmallBitVec::new(475, 10))
        );
        assert_eq!(
            reader.read_bits::<u32>(12).ok(),
            Some(SmallBitVec::new(3784, 12))
        );
    }

    #[test]
    fn rightbitreader_read() {
        let cursor = vec![0b1100_1100];

        let mut reader = BitReader::<Right, _>::new(cursor);

        assert_eq!(
            reader.read_bits::<u32>(1).ok(),
            Some(SmallBitVec::new(0b0, 1))
        );
        assert_eq!(
            reader.read_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(0b10, 2))
        );
        assert_eq!(
            reader.read_bits::<u32>(3).ok(),
            Some(SmallBitVec::new(0b0001, 3))
        );
        assert_eq!(
            reader.read_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(0b11, 2))
        );
    }

    #[test]
    fn rightbitreader_multi() {
        let cursor = vec![0xCF, 0x6F, 0x87, 0xEC];

        let mut reader = BitReader::<Right, _>::new(cursor);

        assert_eq!(
            reader.read_bits::<u32>(10).ok(),
            Some(SmallBitVec::new(975, 10))
        );
        assert_eq!(
            reader.read_bits::<u32>(10).ok(),
            Some(SmallBitVec::new(475, 10))
        );
        assert_eq!(
            reader.read_bits::<u32>(12).ok(),
            Some(SmallBitVec::new(3784, 12))
        );
    }

    #[test]
    fn leftbitreader_peek() {
        let mut writer = BitWriter::<Left>::new();
        let ret = vec![
            SmallBitVec::new(0b1_u32, 1),
            SmallBitVec::new(0b10, 2),
            SmallBitVec::new(0b011, 3),
            SmallBitVec::new(0b00, 2),
        ].to_bytes(&mut writer, Action::Flush)
            .collect::<Vec<_>>();

        let mut reader = BitReader::<Left, _>::new(ret);

        assert_eq!(
            reader.peek_bits::<u32>(1).ok(),
            Some(SmallBitVec::new(0b1, 1))
        );
        assert_eq!(
            reader.read_bits::<u32>(1).ok(),
            Some(SmallBitVec::new(0b1, 1))
        );
        assert_eq!(
            reader.peek_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(0b10, 2))
        );
        assert_eq!(
            reader.read_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(0b10, 2))
        );
        assert_eq!(
            reader.peek_bits::<u32>(3).ok(),
            Some(SmallBitVec::new(0b011, 3))
        );
        assert_eq!(
            reader.read_bits::<u32>(3).ok(),
            Some(SmallBitVec::new(0b011, 3))
        );
        assert_eq!(
            reader.peek_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(0b00, 2))
        );
        assert_eq!(
            reader.read_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(0b00, 2))
        );
    }

    #[test]
    fn leftbitreader_peek_big() {
        let mut writer = BitWriter::<Left>::new();
        let ret = vec![
            SmallBitVec::new(975_u32, 10),
            SmallBitVec::new(475, 10),
            SmallBitVec::new(3784, 12),
        ].to_bytes(&mut writer, Action::Flush)
            .collect::<Vec<_>>();

        let mut reader = BitReader::<Left, _>::new(ret);

        assert_eq!(
            reader.peek_bits::<u32>(10).ok(),
            Some(SmallBitVec::new(975, 10))
        );
        assert_eq!(
            reader.read_bits::<u32>(10).ok(),
            Some(SmallBitVec::new(975, 10))
        );
        assert_eq!(
            reader.peek_bits::<u32>(10).ok(),
            Some(SmallBitVec::new(475, 10))
        );
        assert_eq!(
            reader.read_bits::<u32>(10).ok(),
            Some(SmallBitVec::new(475, 10))
        );
        assert_eq!(
            reader.peek_bits::<u32>(15).ok(),
            Some(SmallBitVec::new(3784, 12))
        );
        assert_eq!(
            reader.read_bits::<u32>(15).ok(),
            Some(SmallBitVec::new(3784, 12))
        );
    }

    #[test]
    fn leftbitreader_zeros() {
        let mut writer = BitWriter::<Left>::new();
        let ret = vec![
            SmallBitVec::new(32_u32, 16),
            SmallBitVec::new(8, 5),
            SmallBitVec::new(0, 3),
            SmallBitVec::new(1, 3),
            SmallBitVec::new(0, 3),
            SmallBitVec::new(3, 2),
            SmallBitVec::new(0, 3),
        ].to_bytes(&mut writer, Action::Flush)
            .collect::<Vec<_>>();

        let mut reader = BitReader::<Left, _>::new(ret);

        assert_eq!(
            reader.read_bits::<u32>(16).ok(),
            Some(SmallBitVec::new(32, 16))
        );
        assert_eq!(
            reader.read_bits::<u32>(5).ok(),
            Some(SmallBitVec::new(8, 5))
        );
        assert_eq!(
            reader.read_bits::<u32>(3).ok(),
            Some(SmallBitVec::new(0, 3))
        );
        assert_eq!(
            reader.read_bits::<u32>(3).ok(),
            Some(SmallBitVec::new(1, 3))
        );
        assert_eq!(
            reader.read_bits::<u32>(3).ok(),
            Some(SmallBitVec::new(0, 3))
        );
        assert_eq!(
            reader.read_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(3, 2))
        );
        assert_eq!(
            reader.read_bits::<u32>(3).ok(),
            Some(SmallBitVec::new(0, 3))
        );
    }

    #[test]
    fn leftbitreader_skip() {
        let mut writer = BitWriter::<Left>::new();
        let ret = vec![
            SmallBitVec::new(0b1_u32, 1),
            SmallBitVec::new(0b10, 2),
            SmallBitVec::new(0b011, 3),
            SmallBitVec::new(0b00, 2),
        ].to_bytes(&mut writer, Action::Flush)
            .collect::<Vec<_>>();

        let mut reader = BitReader::<Left, _>::new(ret);

        assert_eq!(
            reader.peek_bits::<u32>(1).ok(),
            Some(SmallBitVec::new(0b1, 1))
        );
        assert_eq!(reader.skip_bits(1).ok(), Some(1));
        assert_eq!(
            reader.peek_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(0b10, 2))
        );
        assert_eq!(reader.skip_bits(2).ok(), Some(2));
        assert_eq!(
            reader.peek_bits::<u32>(3).ok(),
            Some(SmallBitVec::new(0b011, 3))
        );
        assert_eq!(reader.skip_bits(3).ok(), Some(3));
        assert_eq!(
            reader.peek_bits::<u32>(2).ok(),
            Some(SmallBitVec::new(0b00, 2))
        );
        assert_eq!(reader.skip_to_next_byte(), 2);
    }

    #[test]
    fn leftbitreader_skip_big() {
        let mut writer = BitWriter::<Left>::new();
        let ret = vec![
            SmallBitVec::new(975_u32, 10),
            SmallBitVec::new(475, 10),
            SmallBitVec::new(3784, 12),
        ].to_bytes(&mut writer, Action::Flush)
            .collect::<Vec<_>>();

        let mut reader = BitReader::<Left, _>::new(ret);

        assert_eq!(
            reader.peek_bits::<u32>(10).ok(),
            Some(SmallBitVec::new(975, 10))
        );
        assert_eq!(reader.skip_bits(20).ok(), Some(20));
        assert_eq!(
            reader.peek_bits::<u32>(15).ok(),
            Some(SmallBitVec::new(3784, 12))
        );
        assert_eq!(reader.skip_to_next_byte(), 4);
        assert_eq!(
            reader.peek_bits::<u32>(15).ok(),
            Some(SmallBitVec::new(200, 8))
        );
    }

}
