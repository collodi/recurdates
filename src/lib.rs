#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate chrono;

use chrono::{DateTime, Utc, TimeZone};
use chrono::naive::{MAX_DATE, MIN_DATE};

macro_rules! max_datetime {
    () => (Utc.from_utc_datetime(&MAX_DATE.and_hms(23, 59, 59)))
}

macro_rules! min_datetime {
    () => (Utc.from_utc_datetime(&MIN_DATE.and_hms(0, 0, 0)))
}

#[cfg(test)]
mod tests;
pub mod repeat_every;

use repeat_every::RepeatEvery;

/// `ReDateTime` struct represents recurring datetime object
/// starting at `at` and repeating every `repeat` up to `until` (inclusive).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReDateTime {
    pub at: DateTime<Utc>,
    pub repeat: RepeatEvery,
    pub until: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ReDateTimeIter<'a> {
    from: &'a ReDateTime,
    adder: Option<RepeatEvery>,
}

impl ReDateTime {
    /// `ReDateTime` with no repeat.
    pub fn at(dt: DateTime<Utc>) -> Self {
        ReDateTime { at: dt, repeat: RepeatEvery::zero(), until: max_datetime!() }
    }

    /// `ReDateTime` with infinite repeat.
    pub fn repeat(dt: DateTime<Utc>, dur: RepeatEvery) -> Self {
        let lim = if dur.is_zero() { min_datetime!() } else { max_datetime!()
        };
        ReDateTime { at: dt, repeat: dur, until: lim }
    }

    /// `ReDateTime` with a finite number of repeats.
    pub fn repeat_until(dt: DateTime<Utc>, dur: RepeatEvery, til: DateTime<Utc>) -> Self {
        ReDateTime { at: dt, repeat: dur, until: til }
    }

    /// True if there is no repeat after or on `dt`.
    pub fn done_before(&self, dt: &DateTime<Utc>) -> bool {
        if self.repeat.is_zero() {
            return self.at < *dt;
        }

        self.until != max_datetime!()
            && (self.until < *dt || self.iter().last().unwrap() < *dt)
    }

    /// True if there is a repeat between `df` (inclusive) and `dt` (inclusive). `df` always comes earlier than `dt`.
    pub fn between(&self, df: &DateTime<Utc>, dt: &DateTime<Utc>) -> bool {
        self.first_after(df).map_or(false, |d| d <= *dt)
    }

    /// First repeat after `df` (inclusive).
    pub fn first_after(&self, df: &DateTime<Utc>) -> Option<DateTime<Utc>> {
        if self.repeat.is_zero() {
            if self.at < *df {
                None
            } else {
                Some(self.at)
            }
        } else {
            if self.until < *df {
                None
            } else {
                self.iter().skip_while(|x| *x < *df).next()
            }
        }
    }

    /// An iterator visiting datetimes in order. If `repeat` is negative, the iterator runs in reverse-chronological order.
    pub fn iter(&self) -> ReDateTimeIter {
        let a = if self.repeat.is_zero() {
            None } else {
            Some(RepeatEvery::zero())
        };
        ReDateTimeIter { from: &self, adder: a }
    }
}

impl<'a> Iterator for ReDateTimeIter<'a> {
    type Item = DateTime<Utc>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.adder.is_none() {
            return None;
        }

        let x = self.adder.clone().unwrap();
        self.adder = x.checked_add(&self.from.repeat);
        if let Some(n) = x.add_to(&self.from.at) {
            if n <= self.from.until {
                return Some(n);
            }
        }
        None
    }
}
