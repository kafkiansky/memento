use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::Duration;

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

#[derive(Debug, Clone)]
pub struct Set {
    key: String,
    item: Item,
}

impl Set {
    ///
    /// ```rust
    /// use memento::{Item, Set};
    ///
    /// let key = Set::new("x", Item::timeless("y"));
    /// ```
    pub fn new<T: ToString>(key: T, item: Item) -> Self {
        Self {
            key: key.to_string(),
            item,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Set(Set),
    Stats,
    Get(Vec<String>),
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Self::Set(cmd) => {
                format!(
                    "set {key} {flags} {expires} {len}\r\n{value}\r\n",
                    key = cmd.key,
                    flags = 0,
                    expires = cmd.item.seconds(),
                    len = cmd.item.value.len(),
                    value = cmd.item.value
                )
            }
            Self::Get(cmd) => format!("get {key}\r\n", key = cmd.join(" ")),
            Self::Stats => "stats\r\n".to_string(),
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
}

impl CommandResp {
    pub(crate) fn from_vec(frames: Vec<String>) -> anyhow::Result<Option<Self>> {
        println!("{:#?}", frames);

        if frames.is_empty() {
            return Ok(None);
        }

        Ok(Some(CommandResp::NotFound))
    }
}

#[derive(Debug)]
pub struct InvalidCommandResp(String);

impl Display for InvalidCommandResp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown response {response} received", response = self.0)
    }
}

impl std::error::Error for InvalidCommandResp {}

impl<T> From<Vec<T>> for CommandResp
where
    T: ToString,
{
    fn from(frames: Vec<T>) -> Self {
        if frames.is_empty() {
            return CommandResp::NoResponse;
        }

        return CommandResp::NotFound;
    }
}

impl FromStr for CommandResp {
    type Err = InvalidCommandResp;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "STORED" => Ok(CommandResp::Stored),
            "ERROR" => Ok(CommandResp::Error),
            "NOT_STORED" => Ok(CommandResp::NotStored),
            "EXISTS" => Ok(CommandResp::Exists),
            "NOT_FOUND" => Ok(CommandResp::NotFound),
            response => Err(InvalidCommandResp(response.to_string())),
        }
    }
}
