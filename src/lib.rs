mod command;
mod error;
mod memento;

pub use self::{command::*, error::*, memento::*};

use tokio::net::ToSocketAddrs;

pub type Result<T> = std::result::Result<T, MementoError>;

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

///
/// ```rust
/// let cmd = memento::set("x".parse::<memento::Key>().unwrap(), memento::Item::timeless("y"));
/// ```
pub fn set(key: Key, item: Item) -> Command {
    Command::Set(Set::new(key, item))
}

///
/// ```rust
/// let cmd = memento::get("x");
/// ```
pub fn get<T: ToString>(key: T) -> Command {
    Command::Get(vec![key.to_string()])
}

///
/// ```rust
/// let cmd = memento::gets(vec!["x"]);
/// ```
pub fn gets<T: ToString>(keys: Vec<T>) -> Command {
    Command::Get(
        keys.iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>(),
    )
}

///
/// ```rust
/// let cmd = memento::stats();
/// ```
pub fn stats() -> Command {
    Command::Stats
}
