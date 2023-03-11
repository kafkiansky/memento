use crate::MementoError;
use std::fmt::Debug;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Key {
    value: String,
}

impl ToString for Key {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

///
/// ```rust
/// async fn main() -> memento::Result<()> {
///     let raw_key = "x".parse::<memento::Key>()?; // x
///     let value_key = "VALUE x 0 3".parse::<memento::Key>()?; // x
///
///     Ok(())
/// }
/// ```
impl FromStr for Key {
    type Err = MementoError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.contains("VALUE") {
            return Ok(Key {
                value: value
                    .split_whitespace()
                    .skip(1)
                    .next()
                    .unwrap_or_default()
                    .to_string(),
            });
        }

        if value.len() > 250 {
            return Err(MementoError::TooLongKey(value.to_string()));
        }

        return Ok(Key {
            value: value.to_string(),
        });
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    value: String,
    expires: Option<Duration>,
}

impl Item {
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use memento::Item;
    ///
    /// let item = Item::expires("y", Duration::from_secs(2));
    /// ```
    pub fn expires<T: ToString>(value: T, expires: Duration) -> Self {
        Self {
            value: value.to_string(),
            expires: Some(expires),
        }
    }

    ///
    /// ```rust
    /// use memento::Item;
    ///
    /// let item = Item::timeless("y");
    /// ```
    pub fn timeless<T: ToString>(value: T) -> Self {
        Self {
            value: value.to_string(),
            expires: None,
        }
    }

    fn seconds(&self) -> u64 {
        self.expires.unwrap_or(Duration::from_secs(0)).as_secs()
    }
}

///
/// ```rust
/// async fn main() -> memento::Result<()> {
///     let item = "value".parse::<memento::Item>()?; // value
///
///     Ok(())
/// }
/// ```
impl FromStr for Item {
    type Err = MementoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Item::timeless(s))
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    key: Key,
    item: Item,
}

impl Set {
    ///
    /// ```rust
    /// use memento::{Item, Set};
    ///
    /// let key = Set::new("x".parse::<memento::Key>().unwrap(), Item::timeless("y"));
    /// ```
    pub fn new(key: Key, item: Item) -> Self {
        Self { key, item }
    }
}

#[derive(Debug, Clone)]
pub struct Incr {
    key: Key,
    value: u64,
}

impl Incr {
    ///
    /// ```rust
    /// use memento::Incr;
    ///
    /// let key = Incr::new("x".parse::<memento::Key>().unwrap(), 1);
    /// ```
    pub fn new(key: Key, value: u64) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone)]
pub struct Decr {
    key: Key,
    value: usize,
}

impl Decr {
    ///
    /// ```rust
    /// use memento::Decr;
    ///
    /// let key = Decr::new("x".parse::<memento::Key>().unwrap(), 1);
    /// ```
    pub fn new(key: Key, value: usize) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Set(Set),
    Stats,
    Get(Vec<String>),
    Incr(Incr),
    Decr(Decr),
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Self::Set(cmd) => {
                format!(
                    "set {key} {flags} {expires} {len}\r\n{value}\r\n",
                    key = cmd.key.to_string(),
                    flags = 0,
                    expires = cmd.item.seconds(),
                    len = cmd.item.value.len(),
                    value = cmd.item.value
                )
            }
            Self::Get(cmd) => format!("get {key}\r\n", key = cmd.join(" ")),
            Self::Stats => "stats\r\n".to_string(),
            Self::Incr(cmd) => {
                format!(
                    "incr {key} {value}\r\n",
                    key = cmd.key.to_string(),
                    value = cmd.value
                )
            }
            Self::Decr(cmd) => {
                format!(
                    "decr {key} {value}\r\n",
                    key = cmd.key.to_string(),
                    value = cmd.value
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum CommandResp {
    Stored,
    Error,
    NotStored,
    Exists,
    NotFound,
    NoResponse,
    Value(Vec<(Key, Item)>),
    Stat,
    Incremented(u64),
    Decremented(u64),
}

impl CommandResp {
    pub(crate) fn from_vec<T>(mut frames: Vec<T>, cmd: Command) -> crate::Result<Option<Self>>
    where
        T: ToString + Debug,
    {
        let response = match frames
            .first()
            .map(ToString::to_string)
            .unwrap_or_default()
            .split_whitespace()
            .next()
            .unwrap_or_default()
        {
            "STORED" => Some(CommandResp::Stored),
            "VALUE" => {
                frames.pop(); // remove END keyword.

                let mut values = Vec::default();

                for chunk in frames.chunks(2) {
                    values.push((
                        chunk[0].to_string().as_str().parse::<Key>()?,
                        chunk[1].to_string().as_str().parse::<Item>()?,
                    ));
                }

                Some(CommandResp::Value(values))
            }
            "STAT" => Some(CommandResp::Stat),
            "ERROR" => Some(CommandResp::Error),
            "NOT_FOUND" => Some(CommandResp::NotFound),
            "" => None,
            value => match cmd {
                Command::Incr(..) => Some(CommandResp::Incremented(value.parse::<u64>()?)),
                Command::Decr(..) => Some(CommandResp::Decremented(value.parse::<u64>()?)),
                _ => None,
            },
        };

        Ok(response)
    }
}
