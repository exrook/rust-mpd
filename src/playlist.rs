//! The module defines playlist data structures

use chrono::NaiveDateTime;
use convert::FromMap;
use error::{Error, ProtoError};

use std::collections::BTreeMap;

/// Playlist
#[derive(Clone, Debug, PartialEq)]
pub struct Playlist {
    /// name
    pub name: String,
    /// last modified
    pub last_mod: NaiveDateTime,
}

impl FromMap for Playlist {
    fn from_map(map: BTreeMap<String, String>) -> Result<Playlist, Error> {
        Ok(Playlist {
            name: try!(map.get("playlist").map(|v| v.to_owned()).ok_or(Error::Proto(ProtoError::NoField("playlist")))),
            last_mod: try!(map.get("Last-Modified").ok_or(Error::Proto(ProtoError::NoField("Last-Modified"))).and_then(|v| {
                NaiveDateTime::parse_from_str(&*v, "%Y-%m-%dT%H:%M:%S%Z").map_err(From::from)
            })),
        })
    }
}
