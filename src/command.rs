use crate::{MementoError, ToCommandResponse};
use std::fmt::Debug;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
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
pub struct Add {
    key: Key,
    item: Item,
}

impl Add {
    ///
    /// ```rust
    /// use memento::{Item, Add};
    ///
    /// let key = Add::new("x".parse::<memento::Key>().unwrap(), Item::timeless("y"));
    /// ```
    pub fn new(key: Key, item: Item) -> Self {
        Self { key, item }
    }
}

#[derive(Debug, Clone)]
pub struct Append {
    key: Key,
    item: Item,
}

impl Append {
    ///
    /// ```rust
    /// use memento::{Item, Append};
    ///
    /// let key = Append::new("x".parse::<memento::Key>().unwrap(), Item::timeless("y"));
    /// ```
    pub fn new(key: Key, item: Item) -> Self {
        Self { key, item }
    }
}

#[derive(Debug, Clone)]
pub struct Prepend {
    key: Key,
    item: Item,
}

impl Prepend {
    ///
    /// ```rust
    /// use memento::{Item, Prepend};
    ///
    /// let key = Prepend::new("x".parse::<memento::Key>().unwrap(), Item::timeless("y"));
    /// ```
    pub fn new(key: Key, item: Item) -> Self {
        Self { key, item }
    }
}

#[derive(Debug, Clone)]
pub struct Replace {
    key: Key,
    item: Item,
}

impl Replace {
    ///
    /// ```rust
    /// use memento::{Item, Replace};
    ///
    /// let key = Replace::new("x".parse::<memento::Key>().unwrap(), Item::timeless("y"));
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
    value: u64,
}

impl Decr {
    ///
    /// ```rust
    /// use memento::Decr;
    ///
    /// let key = Decr::new("x".parse::<memento::Key>().unwrap(), 1);
    /// ```
    pub fn new(key: Key, value: u64) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone)]
pub enum Stat {
    /// Process id of this server process.
    Pid(u32),

    /// Number of secs since the server started.
    Uptime(u32),

    /// Current UNIX time according to the server.
    Time(u32),

    /// Version string of this server.
    Version(String),

    /// Default size of pointers on the host OS (generally 32 or 64).
    PointerSize(usize),

    /// Accumulated user time for this process (seconds:microseconds).
    RUsageUser((u32, u32)),

    /// Accumulated system time for this process (seconds:microseconds).  
    RUsageSystem((u32, u32)),

    /// Max number of simultaneous connections.
    MaxConnections(u32),

    /// Number of open connections.
    CurrConnections(u32),

    /// Total number of connections opened since the server started running.
    TotalConnections(u32),

    /// Conns rejected in maxconns_fast mode.
    RejectedConnections(u64),

    /// Number of connection structures allocated by the server.
    ConnectionStructures(u32),

    /// Current number of bytes used to store items.  
    Bytes(u64),

    /// Total number of items stored since the server started.  
    TotalItems(u64),

    /// Current number of items stored.
    CurrItems(u64),

    /// Cumulative number of retrieval reqs.
    CmdGet(u64),

    /// Cumulative number of storage reqs.
    CmdSet(u64),

    /// Number of successful incr reqs.
    IncrHits(u64),

    /// Number of successful decr reqs.
    DecrHits(u64),

    /// Orphan stat.
    Other { name: String, value: String },
}

impl FromStr for Stat {
    type Err = MementoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stat = s.split_whitespace().skip(1).collect::<Vec<&str>>();

        let stat = match stat[0] {
            "pid" => Stat::Pid(stat[1].parse()?),
            "uptime" => Stat::Uptime(stat[1].parse()?),
            "time" => Stat::Time(stat[1].parse()?),
            "version" => Stat::Version(stat[1].to_string()),
            "pointer_size" => Stat::PointerSize(stat[1].parse()?),
            "rusage_user" => {
                let stat = stat[1].split(".").collect::<Vec<&str>>();

                Stat::RUsageUser((stat[0].parse()?, stat[1].parse()?))
            }
            "rusage_system" => {
                let stat = stat[1].split(".").collect::<Vec<&str>>();

                Stat::RUsageSystem((stat[0].parse()?, stat[1].parse()?))
            }
            "max_connections" => Stat::MaxConnections(stat[1].parse()?),
            "curr_connections" => Stat::CurrConnections(stat[1].parse()?),
            "total_connections" => Stat::TotalConnections(stat[1].parse()?),
            "rejected_connections" => Stat::RejectedConnections(stat[1].parse()?),
            "connection_structures" => Stat::ConnectionStructures(stat[1].parse()?),
            "bytes" => Stat::Bytes(stat[1].parse()?),
            "total_items" => Stat::TotalItems(stat[1].parse()?),
            "curr_items" => Stat::CurrItems(stat[1].parse()?),
            "cmd_get" => Stat::CmdGet(stat[1].parse()?),
            "cmd_set" => Stat::CmdSet(stat[1].parse()?),
            "incr_hits" => Stat::IncrHits(stat[1].parse()?),
            "decr_hits" => Stat::DecrHits(stat[1].parse()?),
            _ => Stat::Other {
                name: stat[0].to_string(),
                value: stat[1].to_string(),
            },
        };

        Ok(stat)
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Set(Set),
    Add(Add),
    Append(Append),
    Prepend(Prepend),
    Replace(Replace),
    Stats,
    Get(Key),
    Gets(Vec<Key>),
    Incr(Incr),
    Decr(Decr),
    Delete(Key),
    Version,
    Quit,
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
            Self::Add(cmd) => {
                format!(
                    "add {key} {flags} {expires} {len}\r\n{value}\r\n",
                    key = cmd.key.to_string(),
                    flags = 0,
                    expires = cmd.item.seconds(),
                    len = cmd.item.value.len(),
                    value = cmd.item.value
                )
            }
            Self::Append(cmd) => {
                format!(
                    "append {key} {flags} {expires} {len}\r\n{value}\r\n",
                    key = cmd.key.to_string(),
                    flags = 0,
                    expires = cmd.item.seconds(),
                    len = cmd.item.value.len(),
                    value = cmd.item.value
                )
            }
            Self::Prepend(cmd) => {
                format!(
                    "prepend {key} {flags} {expires} {len}\r\n{value}\r\n",
                    key = cmd.key.to_string(),
                    flags = 0,
                    expires = cmd.item.seconds(),
                    len = cmd.item.value.len(),
                    value = cmd.item.value
                )
            }
            Self::Replace(cmd) => {
                format!(
                    "replace {key} {flags} {expires} {len}\r\n{value}\r\n",
                    key = cmd.key.to_string(),
                    flags = 0,
                    expires = cmd.item.seconds(),
                    len = cmd.item.value.len(),
                    value = cmd.item.value
                )
            }
            Self::Get(key) => format!("get {key}\r\n", key = key.to_string()),
            Self::Gets(cmd) => {
                format!(
                    "gets {key}\r\n",
                    key = cmd
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
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
            Self::Delete(key) => format!("delete {key}\r\n", key = key.to_string()),
            Self::Version => "version\r\n".to_string(),
            Self::Quit => "quit\r\n".to_string(),
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
    Values(Vec<(Key, Item)>),
    Value { key: Key, item: Item },
    Stats(Vec<Stat>),
    Counter(u64),
    Deleted,
    Touched,
    Version(String),
}

impl Default for CommandResp {
    fn default() -> Self {
        Self::NoResponse
    }
}

impl ToCommandResponse for CommandResp {
    fn create<T>(mut frames: Vec<T>, cmd: Command) -> crate::Result<Option<Self>>
    where
        T: ToString + Debug + Default,
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

                match cmd {
                    Command::Get(..) => Some(CommandResp::Value {
                        key: frames[0].to_string().as_str().parse()?,
                        item: frames[1].to_string().as_str().parse()?,
                    }),
                    Command::Gets(..) => {
                        let mut values = Vec::default();

                        for chunk in frames.chunks(2) {
                            values.push((
                                chunk[0].to_string().as_str().parse::<Key>()?,
                                chunk[1].to_string().as_str().parse::<Item>()?,
                            ));
                        }

                        Some(CommandResp::Values(values))
                    }
                    _ => None,
                }
            }
            "STAT" => {
                frames.pop(); // remove END keyword.

                let mut stats = Vec::default();

                for stat in frames {
                    stats.push(stat.to_string().as_str().parse::<Stat>()?);
                }

                Some(CommandResp::Stats(stats))
            }
            "ERROR" => Some(CommandResp::Error),
            "NOT_FOUND" => Some(CommandResp::NotFound),
            "NOT_STORED" => Some(CommandResp::NotStored),
            "DELETED" => Some(CommandResp::Deleted),
            "END" => Some(CommandResp::NotFound),
            "EXISTS" => Some(CommandResp::Exists),
            "TOUCHED" => Some(CommandResp::Touched),
            "VERSION" => Some(CommandResp::Version(
                frames
                    .first()
                    .map(ToString::to_string)
                    .unwrap_or_default()
                    .split_whitespace()
                    .last()
                    .unwrap_or_default()
                    .to_string(),
            )),
            "" => None,
            value => match cmd {
                Command::Incr(..) | Command::Decr(..) => {
                    Some(CommandResp::Counter(value.parse::<u64>()?))
                }
                _ => None,
            },
        };

        Ok(response)
    }
}
