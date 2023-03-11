use crate::{Command, MementoError, ToCommandResponse};
use bytes::{Buf, BytesMut};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub(crate) struct Connection {
    stream: BufStream<TcpStream>,
    buffer: BytesMut,
    cursor: usize,
}

unsafe impl Send for Connection {}

impl Connection {
    /// Connection used by Memento to handle read/write operations.
    /// Uses BufStream with 4KB capacity by default.
    pub(crate) fn from_stream(stream: TcpStream) -> Self {
        Self {
            stream: BufStream::new(stream),
            buffer: BytesMut::with_capacity(4096),
            cursor: 0,
        }
    }

    /// Connect to TcpStream using underlying address that satisfy ToSocketAddrs trait.
    pub(crate) async fn connect<A: ToSocketAddrs>(addr: A) -> crate::Result<Self> {
        Ok(Self::from_stream(TcpStream::connect(addr).await?))
    }

    pub(crate) async fn execute<T: ToCommandResponse>(&mut self, cmd: Command) -> crate::Result<T> {
        self.stream.write_all(cmd.to_string().as_bytes()).await?;
        self.stream.flush().await?;

        self.read_response(cmd).await
    }

    async fn read_response<T: ToCommandResponse>(&mut self, cmd: Command) -> crate::Result<T> {
        loop {
            if let Some(resp) = self.parse_response(cmd.clone()).await? {
                return Ok(resp);
            }

            if self.buffer.len() == self.cursor {
                self.buffer.resize(self.cursor * 2, 0);
            }

            let len = self.stream.read_buf(&mut self.buffer).await?;

            if 0 == len {
                if self.cursor == 0 {
                    return Ok(T::default());
                }

                return Err(MementoError::ConnectionReset);
            }

            self.cursor += len;
        }
    }

    async fn parse_response<T: ToCommandResponse>(
        &mut self,
        cmd: Command,
    ) -> crate::Result<Option<T>> {
        let mut frames: Vec<String> = Vec::new();

        let mut lines = self.buffer.lines();

        let mut frame_len = 0;

        while let Some(line) = lines.next_line().await? {
            frame_len += line.len() + 2;
            frames.push(line);
        }

        self.buffer.advance(frame_len);

        T::create(frames, cmd)
    }
}
