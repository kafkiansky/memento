extern crate core;

mod command;
mod connection;
mod error;
mod memento;

pub use self::{command::*, error::*, memento::*};
use std::fmt::Debug;

use tokio::net::ToSocketAddrs;

pub type Result<T> = std::result::Result<T, MementoError>;

pub trait ToCommandResponse: Default {
    fn create<T>(frames: Vec<T>, cmd: Command) -> Result<Option<Self>>
    where
        T: ToString + Debug + Default;
}

///
/// ```rust
/// use memento::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = memento::new("localhost:11211").await?;
///
///     Ok(())
/// }
///```
pub async fn new<A: ToSocketAddrs>(addr: A) -> Result<Memento> {
    Memento::connect(addr).await
}
