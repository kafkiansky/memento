mod command;
mod memento;

pub use self::{command::*, memento::*};

use tokio::net::ToSocketAddrs;

///
/// ```rust
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), dyn Error> {
///     let client = memento::new("localhost:11211").await?;
///
///     Ok(())
/// }
///```
pub async fn new<A: ToSocketAddrs>(addr: A) -> anyhow::Result<Memento> {
    Memento::connect(addr).await
}

///
/// ```rust
/// let cmd = memento::set("x", memento::Item::timeless("y"));
/// ```
pub fn set<T: ToString>(name: T, item: Item) -> Command {
    Command::Set(Set::new(name, item))
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
