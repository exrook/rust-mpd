use chrono::{Duration, NaiveDateTime};
use serde::{Serialize, Serializer};
pub(crate) mod duration_tuple_secs {
    use super::DurationWrapper;
    use chrono::Duration;
    use serde::{Serialize, Serializer};
    pub fn serialize<S>(dur: &Option<(Duration, Duration)>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *dur {
            Some((ref a, ref b)) => serializer.serialize_some(&(DurationWrapper(a), DurationWrapper(b))),
            None => serializer.serialize_none(),
        }
    }
}

pub(crate) struct DurationWrapper<'a>(&'a Duration);
impl<'a> Serialize for DurationWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration_secs::serialize(self.0, serializer)
    }
}

pub(crate) mod duration_secs {
    use chrono::Duration;
    use serde::{Serialize, Serializer};
    pub fn serialize<S>(dur: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(dur.num_seconds())
    }
}

pub(crate) mod duration_option_secs {
    use super::DurationWrapper;
    use chrono::Duration;
    use serde::{Serialize, Serializer};
    pub fn serialize<S>(dur: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *dur {
            Some(ref dur) => serializer.serialize_some(&DurationWrapper(dur)),
            None => serializer.serialize_none(),
        }
    }
}



pub(crate) struct NaiveDateTimeWrapper<'a>(&'a NaiveDateTime);
impl<'a> Serialize for NaiveDateTimeWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        time_secs::serialize(self.0, serializer)
    }
}

pub(crate) mod time_secs {
    use chrono::NaiveDateTime;
    use serde::{Serialize, Serializer};
    pub fn serialize<S>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(time.timestamp())
    }
}

pub(crate) mod time_option_secs {
    use super::NaiveDateTimeWrapper;
    use chrono::NaiveDateTime;
    use serde::{Serialize, Serializer};
    pub fn serialize<S>(time: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *time {
            Some(ref time) => serializer.serialize_some(&NaiveDateTimeWrapper(time)),
            None => serializer.serialize_none(),
        }
    }
}
