mod parse;

use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use super::serde::*;

pub use parse::parse;

#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, Clone, PartialEq, Serialize, Default))]
pub struct Config {
    /// The required version of kitty
    pub gg: Option<String>,
    #[serde(deserialize_with = "de_string_or_struct_hashmap", default = "HashMap::new")]
    pub scripts: HashMap<String, Script>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, Clone, PartialEq, Serialize, Default))]
pub struct Script {}

impl FromStr for Script {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Script {})
    }
}