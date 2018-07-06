use std::cmp;
use std::ops::Add;
use chrono::{DateTime, Utc, Duration, Datelike};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepeatEvery {
    minutes: u32,
    days: u32,
    months: u32,
}

/// Add two `struct`s.
impl<'a> Add<&'a Self> for RepeatEvery {
    type Output = RepeatEvery;

    fn add(self, rhs: &Self) -> Self::Output {
        RepeatEvery {
            minutes: self.minutes + rhs.minutes,
            days: self.days + rhs.days,
            months: self.months + rhs.months,
        }
    }
}

impl RepeatEvery {
    /// `RepeatEvery` where all fields are zero.
    pub fn zero() -> Self {
        RepeatEvery { minutes: 0, days: 0, months: 0 }
    }

    pub fn minutes(n: u32) -> Self {
        RepeatEvery { minutes: n, days: 0, months: 0 }
    }

    pub fn hours(n: u32) -> Self {
        RepeatEvery { minutes: 60 * n, days: 0, months: 0 }
    }

    pub fn days(n: u32) -> Self {
        RepeatEvery { minutes: 0, days: n, months: 0 }
    }

    pub fn weeks(n: u32) -> Self {
        RepeatEvery { minutes: 0, days: n * 7, months: 0 }
    }

    pub fn months(n: u32) -> Self {
        RepeatEvery { minutes: 0, days: 0, months: n }
    }

    pub fn years(n: u32) -> Self {
        RepeatEvery { minutes: 0, days: 0, months: n * 12 }
    }

    /// True if `self` == `RepeatEvery::zero()`.
    pub fn is_zero(&self) -> bool {
        self.minutes == 0 && self.days == 0 && self.months == 0
    }

    /// `dt + self`. `None` if overflow happens.
    pub fn add_to(&self, dt: &DateTime<Utc>) -> Option<DateTime<Utc>> {
        let a = *dt
            + Duration::days(self.days as i64)
            + Duration::minutes(self.minutes as i64);

        if self.months > 0 {
            let y = a.year() + (((dt.month0() + self.months) / 12) as i32);
            let m = (dt.month0() + self.months) % 12;
            a.with_year(y).and_then(|x| x.with_month0(m))
        } else {
            Some(a)
        }
    }

    /// `rhs + self`. `None` if overflow happens.
    pub fn checked_add(&self, rhs: &Self) -> Option<Self> {
        let added = self.clone() + rhs;
        if added.minutes < cmp::max(self.minutes, rhs.minutes) || added.days < cmp::max(self.days, rhs.days) || added.months < cmp::max(self.months, rhs.months) {
            None
        } else {
            Some(added)
        }
    }
}
