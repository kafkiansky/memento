use crate::{Command, CommandResp};
use bytes::{Buf, BytesMut};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub struct Memento {
    stream: BufStream<TcpStream>,
    buffer: BytesMut,
    cursor: usize,
}

impl Memento {
    ///
    /// ```rust
    /// use tokio::net::TcpStream;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), dyn Error> {
    ///     let memento = memento::Memento::from_stream(TcpStream::connect("localhost:11211").await?);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_stream(stream: TcpStream) -> Self {
        Self {
            stream: BufStream::new(stream),
            buffer: BytesMut::with_capacity(4096),
            cursor: 0,
        }
    }

    ///
    /// ```rust
    /// use tokio::net::TcpStream;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), dyn Error> {
    ///     let memento = memento::Memento::connect("localhost:11211").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> anyhow::Result<Self> {
        Ok(Self::from_stream(TcpStream::connect(addr).await?))
    }

    ///
    /// ```rust
    /// use std::error::Error;
    ///
    /// async fn main() -> Result<(), dyn Error> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     let response = memento.execute(memento::set("x", memento::Item::timeless("y"))).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute(&mut self, cmd: Command) -> anyhow::Result<CommandResp> {
        self.stream.write_all(cmd.to_string().as_bytes()).await?;
        self.stream.flush().await?;

        loop {
            if let Some(resp) = self.parse_resp().await? {
                return Ok(resp);
            }

            if self.buffer.len() == self.cursor {
                self.buffer.resize(self.cursor * 2, 0);
            }

            let len = self.stream.read_buf(&mut self.buffer).await?;

            if 0 == len {
                if self.cursor == 0 {
                    return Ok(CommandResp::NoResponse);
                }

                return Err(anyhow::Error::msg("connection reset by peer"));
            }

            self.cursor += len;
        }
    }

    async fn parse_resp(&mut self) -> anyhow::Result<Option<CommandResp>> {
        let mut frames: Vec<String> = Vec::new();

        let mut lines = self.buffer.lines();

        let mut frame_len = 0;

        while let Some(line) = lines.next_line().await? {
            frame_len += line.len() + 2;
            frames.push(line);
        }

        self.buffer.advance(frame_len);

        CommandResp::from_vec(frames)
    }
}
