use crate::connection::Connection;
use crate::{
    Add, Append, Command, CommandResp, Decr, Incr, Item, Key, Prepend, Replace, Set,
    ToCommandResponse,
};
use tokio::net::{TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub struct Memento {
    connection: Connection,
}

unsafe impl Send for Memento {}

impl Memento {
    ///
    /// ```rust
    /// use tokio::net::TcpStream;
    ///
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let memento = memento::Memento::from_stream(TcpStream::connect("localhost:11211").await?);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_stream(stream: TcpStream) -> Self {
        Self {
            connection: Connection::from_stream(stream),
        }
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let memento = memento::Memento::connect("localhost:11211").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> crate::Result<Self> {
        Ok(Self {
            connection: Connection::connect(addr).await?,
        })
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.set("x".parse()?, memento::Item::timeless("y")).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn set(&mut self, key: Key, item: Item) -> crate::Result<CommandResp> {
        self.call(Command::Set(Set::new(key, item))).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.add("x".parse()?, memento::Item::timeless("y")).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn add(&mut self, key: Key, item: Item) -> crate::Result<CommandResp> {
        self.call(Command::Add(Add::new(key, item))).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.append("x".parse()?, memento::Item::timeless("y")).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn append(&mut self, key: Key, item: Item) -> crate::Result<CommandResp> {
        self.call(Command::Append(Append::new(key, item))).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.prepend("x".parse()?, memento::Item::timeless("y")).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn prepend(&mut self, key: Key, item: Item) -> crate::Result<CommandResp> {
        self.call(Command::Prepend(Prepend::new(key, item))).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     let response = memento.get("x".parse()?).await?;
    ///
    ///     match response {
    ///         memento::CommandResp::Values(values) => {
    ///             for (key, value) in values {
    ///                 println!("{key}: {value}", key = key.to_string(), value = value.to_string())
    ///             }
    ///         },
    ///         _ => {},
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&mut self, key: Key) -> crate::Result<CommandResp> {
        self.call(Command::Get(key)).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     let response = memento.gets(vec!["x".parse()?]).await?;
    ///
    ///     match response {
    ///         memento::CommandResp::Values(values) => {
    ///             for (key, value) in values {
    ///                 println!("{key}: {value}", key = key.to_string(), value = value.to_string())
    ///             }
    ///         },
    ///         _ => {},
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn gets(&mut self, keys: Vec<Key>) -> crate::Result<CommandResp> {
        self.call(Command::Gets(keys)).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.incr("x".parse()?, 1).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn incr(&mut self, key: Key, value: u64) -> crate::Result<CommandResp> {
        self.call(Command::Incr(Incr::new(key, value))).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.decr("x".parse()?, 1).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn decr(&mut self, key: Key, value: u64) -> crate::Result<CommandResp> {
        self.call(Command::Decr(Decr::new(key, value))).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.delete("x".parse()?).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&mut self, key: Key) -> crate::Result<CommandResp> {
        self.call(Command::Delete(key)).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.replace("x".parse()?, memento::Item::timeless("y")).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn replace(&mut self, key: Key, item: Item) -> crate::Result<CommandResp> {
        self.call(Command::Replace(Replace::new(key, item))).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.version().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn version(&mut self) -> crate::Result<CommandResp> {
        self.call(Command::Version).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.quit().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn quit(&mut self) -> crate::Result<CommandResp> {
        self.call(Command::Quit).await
    }

    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     memento.stats().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn stats(&mut self) -> crate::Result<CommandResp> {
        self.call(Command::Stats).await
    }

    ///
    /// ```rust
    /// use tokio::net::TcpStream;
    /// use memento::{Command, Incr, CommandResp};
    ///
    /// #[tokio::main]
    /// async fn main() -> memento::Result<()> {
    ///     let mut memento = memento::new("localhost:11211").await?;
    ///
    ///     // or your own implementation of ToCommandResponse trait.
    ///     let response = memento.call::<CommandResp>(Command::Incr(Incr::new("x".parse()?, 1))).await?;
    ///
    ///     println!("{:#?}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn call<T: ToCommandResponse>(&mut self, cmd: Command) -> crate::Result<T> {
        self.connection.execute(cmd).await
    }
}
