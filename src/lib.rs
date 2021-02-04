//! [`Read`]/[`Write`] implementation that Xor's the bytes that come through it and wraps around
//! another [`Read`] or [`Write`].
//!
//! # Examples
//!
//! ## Writing
//!
//! ```no_run
//! # use xorio::Xor;
//! # use std::fs::File;
//! # use std::io::Write;
//! let mut file = File::create("my_xored_file.bin").unwrap();
//! let mut writer = Xor::new(file);
//! writer.write_all("Hello World".as_bytes());
//! ```
//!
//! ## Reading
//!
//! ```no_run
//! # use xorio::Xor;
//! # use std::fs::File;
//! # use std::io::Read;
//! let mut file = File::open("my_xored_file.bin").unwrap();
//! let mut reader = Xor::new(file);
//! let mut content = String::new();
//! reader.read_to_string(&mut content);
//! ```
//!
//! ## Custom Xor Bytes
//!
//! You can also customize the bytes that it will XOR the stream with. By default it uses a single
//! byte `0b01010101` to calculate the XOR.
//!
//! ```no_run
//! # use xorio::Xor;
//! # use std::fs::File;
//! # use std::io::Write;
//! let mut file = File::create("my_xored_file.bin").unwrap();
//! let mut writer = Xor::new_with_xor_bytes(file, vec![1, 2, 3]);
//! writer.write_all("Hello World".as_bytes());
//! ```
//!
//! # License
//!
//! This crate is licensed under the [Katharos License][k_license] which places certain
//! restrictions on what you are allowed to use it for. Please read and understand the terms before
//! using this crate for your project.
//!
//! [k_license]: https://github.com/katharostech/katharos-license

use std::io::{Read, Seek, Write};

/// [`Read`]/[`Write`] implementation that Xor's the bytes that come through it and wraps around
/// another [`Read`] or [`Write`].
pub struct Xor<T> {
    inner: T,
    xor_bytes: Vec<u8>,
}

impl<T> Xor<T> {
    /// Create a new [`Xor`] wrapped around the given IO type.
    pub fn new(inner: T) -> Self {
        Xor {
            inner,
            xor_bytes: vec![0b01010101],
        }
    }

    /// Create a new [`Xor`] wrapped around the given IO type, with custom `xor_bytes` that will be
    /// used when XOR-ing the stream. The default `xor_bytes` is one `0b01010101` byte.
    pub fn new_with_xor_bytes(inner: T, xor_bytes: Vec<u8>) -> Self {
        Xor { inner, xor_bytes }
    }
}

impl<R: Read> Read for Xor<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Read bytes into buf
        let count = self.inner.read(buf)?;

        // Xor the bytes in the buffer
        for byte in buf {
            for xor_byte in &self.xor_bytes {
                *byte = *byte ^ xor_byte;
            }
        }

        Ok(count)
    }
}

impl<W: Write> Write for Xor<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(
            // Write the buffer with each byte XOR-ed
            buf.iter()
                .map(|x| {
                    let mut byte = *x;
                    for xor_byte in &self.xor_bytes {
                        byte = byte ^ xor_byte
                    }
                    byte
                })
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl<S: Seek> Seek for Xor<S> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}
