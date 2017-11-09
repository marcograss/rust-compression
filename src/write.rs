//! rust-compression
//!
//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! <http://mozilla.org/MPL/2.0/>.

use std::io::{Error, ErrorKind, Result};

pub trait Write<T> {
    fn write(&mut self, buf: &T) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_arr(&mut self, buf: &[T]) -> Result<usize> {
        for (i, d) in buf.into_iter().enumerate() {
            match self.write(d) {
                Ok(0) => {
                    return Err(Error::new(
                        ErrorKind::WriteZero,
                        "failed to write whole buffer",
                    ))
                }
                Ok(_) => {}
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                    return Ok(i)
                }
                Err(e) => return Err(e),
            }
        }
        Ok(buf.len())
    }

    fn write_all(&mut self, mut buf: &[T]) -> Result<()> {
        while !buf.is_empty() {
            match self.write_arr(buf) {
                Ok(0) => {
                    return Err(Error::new(
                        ErrorKind::WriteZero,
                        "failed to write whole buffer",
                    ))
                }
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    #[inline]
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}

impl Write<u8> for ::std::io::Write {
    #[inline]
    fn write(&mut self, buf: &u8) -> Result<usize> {
        ::std::io::Write::write(self, &[*buf])
    }

    #[inline]
    fn write_arr(&mut self, buf: &[u8]) -> Result<usize> {
        ::std::io::Write::write(self, buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        ::std::io::Write::flush(self)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        ::std::io::Write::write_all(self, buf)
    }
}

impl<T: Clone> Write<T> for Vec<T> {
    #[inline]
    fn write(&mut self, buf: &T) -> Result<usize> {
        self.push(buf.clone());
        Ok(1)
    }

    #[inline]
    fn write_arr(&mut self, buf: &[T]) -> Result<usize> {
        self.append(&mut buf.to_vec());
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
