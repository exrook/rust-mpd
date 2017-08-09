//! The module describes DB and playback statistics

use chrono::{Duration, NaiveDateTime};
use convert::FromIter;

use error::Error;
use format::{duration_secs, time_secs};
use serde::{Serialize, Serializer};

/// DB and playback statistics
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Stats {
    /// number of artists in DB
    pub artists: u32,
    /// number of albums in DB
    pub albums: u32,
    /// number of songs in DB
    pub songs: u32,
    /// total MPD uptime, seconds resolution
    #[serde(with = "duration_secs")]
    pub uptime: Duration,
    /// total playback time, seconds resolution
    #[serde(with = "duration_secs")]
    pub playtime: Duration,
    /// total playback time for all songs in DB, seconds resolution
    #[serde(with = "duration_secs")]
    pub db_playtime: Duration,
    /// last DB update timestamp, seconds resolution
    #[serde(with = "time_secs")]
    pub db_update: NaiveDateTime,
}

impl Default for Stats {
    fn default() -> Stats {
        Stats {
            artists: 0,
            albums: 0,
            songs: 0,
            uptime: Duration::seconds(0),
            playtime: Duration::seconds(0),
            db_playtime: Duration::seconds(0),
            db_update: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl FromIter for Stats {
    /// build stats from iterator
    fn from_iter<I: Iterator<Item = Result<(String, String), Error>>>(iter: I) -> Result<Stats, Error> {
        let mut result = Stats::default();

        for res in iter {
            let line = try!(res);
            match &*line.0 {
                "artists" => result.artists = try!(line.1.parse()),
                "albums" => result.albums = try!(line.1.parse()),
                "songs" => result.songs = try!(line.1.parse()),
                "uptime" => result.uptime = Duration::seconds(try!(line.1.parse())),
                "playtime" => result.playtime = Duration::seconds(try!(line.1.parse())),
                "db_playtime" => result.db_playtime = Duration::seconds(try!(line.1.parse())),
                "db_update" => result.db_update = NaiveDateTime::from_timestamp(try!(line.1.parse()), 0),
                _ => (),
            }
        }

        Ok(result)
    }
}
