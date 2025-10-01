use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpStream},
};

use anyhow::anyhow;

pub mod chesstp;
pub mod setup;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Server,
    Client,
}

#[derive(Debug)]
pub enum GameConnection {
    Local,
    Remote(ConnectionType, SocketAddr, ChesstpMessageStream),
}

#[derive(Debug)]
pub struct ChesstpMessageStream {
    reader: BufReader<TcpStream>,
    writer: TcpStream,
}

impl ChesstpMessageStream {
    pub fn new(stream: TcpStream) -> anyhow::Result<Self> {
        stream.set_nonblocking(true)?;
        let reader = BufReader::new(stream.try_clone()?);

        Ok(Self {
            reader,
            writer: stream,
        })
    }

    /// Read chesstp message from connection, returning `None` if there isn't enough data available
    /// yet. Is meant to be called in a loop, only returning a message occasionally.
    pub fn accept(&mut self) -> anyhow::Result<Option<chesstp::Message>> {
        if skip_until_slice(&mut self.reader, b"Chess")?.is_none() {
            // Couldn't find prefix yet.
            return Ok(None);
        }

        let mut message_buf = [0u8; 128];
        match self.reader.read_exact(&mut message_buf) {
            Ok(_) => {}
            Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => {
                // There isn't enough data to read currently.
                return Ok(None);
            }
            Err(error) => return Err(error.into()),
        };
        match chesstp::Message::parse_from(&message_buf) {
            Ok(message) => Ok(Some(message)),
            Err(error) => Err(anyhow!("Couldn't parse message: {:?}", error)),
        }
    }

    pub fn write(&mut self, message: chesstp::Message) -> anyhow::Result<()> {
        let written_len = self.writer.write(&message.serialize())?;

        if written_len != 128 {
            return Err(anyhow!("Could only read {} bytes of message", written_len));
        }

        Ok(())
    }
}

/// Consume BufRead until slice has been found, without consuming the slice. Returns None if the
/// operation would block while the underlying buffer was configured to be non-blocking.
pub fn skip_until_slice(reader: &mut impl BufRead, slice: &[u8]) -> io::Result<Option<()>> {
    loop {
        let buf = match reader.fill_buf() {
            Ok(buf) => buf,
            Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => {
                // There isn't more data to read currently.
                return Ok(None);
            }
            Err(error) => return Err(error),
        };
        if buf.is_empty() {
            if buf.is_empty() {
                return Ok(None);
            } else {
                return Ok(Some(()));
            }
        }

        // Search for slice in current buf
        if let Some(pos) = buf.windows(slice.len()).position(|window| window == slice) {
            // Consume all bytes up to the slice.
            reader.consume(pos);
            return Ok(Some(()));
        }

        // The slice wasn't found. If the end of the current buf contains part of the slice, we
        // should some amount of bytes from the current buf so that the maximum length subslice
        // would fit in it, i.e. (slice.len() - 1)
        let keep = slice.len().saturating_sub(1);
        let advance = buf.len().saturating_sub(keep);

        // If advance is 0 (i.e. if the buffer is smaller than the delimiter), still make progress.
        let advance = if advance == 0 {
            buf.len().min(1)
        } else {
            advance
        };

        reader.consume(advance);
    }
}
