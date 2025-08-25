use crate::csharp::TimeSpan;
use crate::exception;
use crate::shared::XnaResult;
use crate::shared::Exception;

const TIME_SPAN_TOO_LONG : &'static str = "TimeSpan overflowed because the duration is too long";
const TIME_SPAN_EXCEED_MAX: &'static str = "The duration cannot be returned for TimeSpan.MinValue because the absolute value of TimeSpan.MinValue exceeds the value of TimeSpan.MaxValue.";

impl TimeSpan {
    pub fn new(ticks: i64) -> Self {
        TimeSpan { ticks }
    }

    pub fn from_time(days: i32, hours: i32, minutes: i32, seconds: i32, milliseconds: i32, microseconds: i32) -> XnaResult<Self> {
        let total_micro_seconds =  (days as i64 * Self::MICROSECONDS_PER_DAY)
            + (hours as i64 * Self::MICROSECONDS_PER_HOUR)
            + (minutes as i64 * Self::MICROSECONDS_PER_MINUTE)
            + (seconds as i64 * Self::MICROSECONDS_PER_SECOND)
            + (milliseconds as i64 * Self::MICROSECONDS_PER_MILLISECOND)
            + microseconds as i64;

        if total_micro_seconds > Self::MAX_MICROSECONDS || total_micro_seconds < Self::MIN_MICROSECONDS {
            return Err(exception!(TIME_SPAN_TOO_LONG, None));
        }

        Ok(TimeSpan { ticks: total_micro_seconds * Self::TICKS_PER_MICROSECOND})
    }

    pub fn max_value() -> Self {
        TimeSpan { ticks: Self::MAX_TICKS }
    }

    pub fn min_value() -> Self {
        TimeSpan { ticks: Self::MIN_TICKS }
    }

    pub fn days(&self) -> i32 {
        (self.ticks / (Self::TICKS_PER_DAY)) as i32
    }

    pub fn hours(&self) -> i32 {
        (self.ticks / Self::TICKS_PER_HOUR % Self::HOURS_PER_DAY as i64) as i32
    }

    pub fn milliseconds(&self) -> i32 {
        (self.ticks / Self::TICKS_PER_MILLISECOND % Self::MILLISECONDS_PER_SECOND) as i32
    }

    pub fn microseconds(&self) -> i32 {
        (self.ticks / Self::TICKS_PER_MICROSECOND % Self::MICROSECONDS_PER_MILLISECOND) as i32
    }

    pub fn nanoseconds(&self) -> i32 {
        (self.ticks / Self::TICKS_PER_MICROSECOND * Self::NANOSECONDS_PER_TICK) as i32
    }

    //#[deny(unconditional_panic)]
    pub fn minutes(&self) -> i32 {
        (self.ticks / Self::TICKS_PER_MINUTE % Self::MINUTES_PER_HOUR) as i32
    }

    pub fn seconds(&self) -> i32 {
        (self.ticks / Self::TICKS_PER_SECOND % Self::SECONDS_PER_MINUTE) as i32
    }

    pub fn total_days(&self) -> f64 {
        (self.ticks / (Self::TICKS_PER_DAY)) as f64
    }

    pub fn total_hours(&self) -> f64 {
        (self.ticks / (Self::TICKS_PER_HOUR)) as f64
    }

    pub fn total_milliseconds(&self) -> f64 {
        let temp = (self.ticks / Self::TICKS_PER_MILLISECOND);

        if temp > Self::MAX_MILLISECONDS {
            return Self::MAX_MILLISECONDS as f64;
        }

        if temp < Self::MIN_MILLISECONDS {
            return Self::MIN_MILLISECONDS as f64;
        }

        temp as f64
    }

    pub fn total_microseconds(&self) -> f64 {
        (self.ticks / Self::TICKS_PER_MICROSECOND) as f64
    }

    pub fn total_nanoseconds(&self) -> f64 {
        (self.ticks / Self::NANOSECONDS_PER_TICK) as f64
    }

    pub fn total_minutes(&self) -> f64 {
        (self.ticks / Self::TICKS_PER_MINUTE) as f64
    }

    pub fn total_seconds(&self) -> f64 {
        (self.ticks / Self::TICKS_PER_SECOND) as f64
    }

    pub fn add(&self, time_span: &TimeSpan) -> Self {
        TimeSpan {ticks: self.ticks + time_span.ticks }
    }

    pub fn subtract(&self, time_span: &TimeSpan) -> Self {
        TimeSpan {ticks: self.ticks - time_span.ticks }
    }

    pub fn multiply(&self, factor: f64) -> Self {
        TimeSpan {ticks: (self.ticks as f64 * factor) as i64 }
    }

    pub fn divide(&self, divisor: f64) -> Self {
        TimeSpan {ticks: (self.ticks as f64 / divisor) as i64 }
    }

    pub fn divide_timespan(&self, ts: &TimeSpan) -> Self {
        TimeSpan {ticks: self.ticks / ts.ticks }
    }

    pub fn negate(&self) -> Self {
        TimeSpan { ticks: -self.ticks }
    }

    pub fn from_days_f(value: f64) -> Result<Self, Exception> {
        Self::interval(value, Self::TICKS_PER_DAY as f64)
    }

    pub fn duration(&self) -> Result<Self, Exception> {
        if self.ticks == Self::MIN_TICKS {
            return Err(exception!(TIME_SPAN_EXCEED_MAX, None));
        }

        Ok(TimeSpan {ticks: if self.ticks >= 0 { self.ticks} else { -self.ticks}})
    }

    pub fn from_days(days: i32) -> Result<Self, Exception> {
        Self::from_units(days as i64, Self::TICKS_PER_DAY, Self::MIN_DAYS, Self::MAX_DAYS)
    }

    pub fn from_hours(hours: i32) -> Result<Self, Exception> {
        Self::from_units(hours as i64, Self::TICKS_PER_HOUR, Self::MIN_HOURS, Self::MAX_HOURS)
    }

    pub fn from_minutes(minutes: i32) -> Result<Self, Exception> {
        Self::from_units(minutes as i64, Self::TICKS_PER_MINUTE, Self::MIN_MINUTES, Self::MAX_MINUTES)
    }

    pub fn from_seconds(seconds: i32) -> Result<Self, Exception> {
        Self::from_units(seconds as i64, Self::TICKS_PER_SECOND, Self::MIN_SECONDS, Self::MAX_SECONDS)
    }

    pub fn from_microseconds(microseconds: i32) -> Result<Self, Exception> {
        Self::from_units(microseconds as i64, Self::TICKS_PER_MICROSECOND, Self::MIN_MICROSECONDS, Self::MAX_MICROSECONDS)
    }

    pub fn from_minutes_f(value: f64) -> Result<Self, Exception> {
        Self::interval(value, Self::TICKS_PER_MICROSECOND as f64)
    }

    pub fn from_ticks(value: i64) -> Self {
        TimeSpan{ ticks: value }
    }

    fn from_units(units: i64, ticks_per_unit: i64, min_units: i64, max_units: i64) -> Result<TimeSpan, Exception> {
        if units > max_units || units < min_units {
            return Err(exception!(TIME_SPAN_TOO_LONG, None));
        }

        Ok(TimeSpan::from_ticks(units * ticks_per_unit))
    }

    fn interval(value: f64, scale: f64) -> Result<Self, Exception> {
        Self::interval_from_double_ticks(value * scale)
    }

    fn interval_from_double_ticks(ticks: f64) -> Result<Self, Exception> {
        if ticks > Self::MAX_TICKS as f64 || ticks < Self::MIN_TICKS as f64 {
            return Err(exception!("TimeSpan overflowed because the duration is too long", None));
        }

        if ticks == Self::MAX_TICKS as f64 {
            return Ok(Self::max_value());
        }

        Ok(TimeSpan { ticks: ticks as i64 })
    }

    pub const NANOSECONDS_PER_TICK: i64 = 100;
    pub const TICKS_PER_MICROSECOND: i64 = 10;
    pub const TICKS_PER_MILLISECOND: i64 = Self::TICKS_PER_MICROSECOND * 1000;
    pub const TICKS_PER_SECOND: i64 = Self::TICKS_PER_MILLISECOND * 1000;
    pub const TICKS_PER_MINUTE: i64 = Self::TICKS_PER_SECOND * 60;
    pub const TICKS_PER_HOUR: i64 = Self::TICKS_PER_MINUTE * 60;
    pub const TICKS_PER_DAY: i64 = 36_000_000_000;
    pub const TICKS_PER_TENTH_SECOND: i64 = Self::TICKS_PER_MILLISECOND * 100;

    pub const MICROSECONDS_PER_MILLISECOND: i64 = Self::TICKS_PER_MILLISECOND / Self::TICKS_PER_MICROSECOND;
    pub const MICROSECONDS_PER_SECOND: i64 = Self::TICKS_PER_SECOND / Self::TICKS_PER_MICROSECOND;
    pub const MICROSECONDS_PER_MINUTE: i64 = Self::TICKS_PER_MINUTE / Self::TICKS_PER_MICROSECOND;
    pub const MICROSECONDS_PER_HOUR: i64 = Self::TICKS_PER_HOUR / Self::TICKS_PER_MICROSECOND;
    pub const MICROSECONDS_PER_DAY: i64 = Self::TICKS_PER_DAY / Self::TICKS_PER_MICROSECOND;

    pub const MILLISECONDS_PER_SECOND: i64 = Self::TICKS_PER_SECOND / Self::TICKS_PER_MILLISECOND;
    pub const MILLISECONDS_PER_MINUTE: i64 = Self::TICKS_PER_MINUTE / Self::TICKS_PER_MILLISECOND;
    pub const MILLISECONDS_PER_HOUR: i64 = Self::TICKS_PER_HOUR / Self::TICKS_PER_MILLISECOND;
    pub const MILLISECONDS_PER_DAY: i64 = Self::TICKS_PER_DAY / Self::TICKS_PER_MILLISECOND;

    pub const SECONDS_PER_MINUTE: i64 = Self::TICKS_PER_MINUTE / Self::TICKS_PER_SECOND;
    pub const SECONDS_PER_HOUR: i64 = Self::TICKS_PER_HOUR / Self::TICKS_PER_SECOND;
    pub const SECONDS_PER_DAY: i64 = Self::TICKS_PER_DAY / Self::TICKS_PER_SECOND;

    pub const MINUTES_PER_HOUR: i64 = Self::TICKS_PER_HOUR / Self::TICKS_PER_MINUTE;
    pub const MINUTES_PER_DAY: i64 = Self::TICKS_PER_DAY / Self::TICKS_PER_MINUTE;

    pub const HOURS_PER_DAY: i32 = 24;

    pub const MIN_TICKS: i64 = i64::MIN;
    pub const MAX_TICKS: i64 = i64::MAX;

    pub const MIN_MICROSECONDS: i64 = Self::MIN_TICKS / Self::TICKS_PER_MICROSECOND;
    pub const MAX_MICROSECONDS: i64 = Self::MAX_TICKS / Self::TICKS_PER_MICROSECOND;

    pub const MIN_MILLISECONDS: i64 = Self::MIN_TICKS / Self::TICKS_PER_MILLISECOND;
    pub const MAX_MILLISECONDS: i64 = Self::MAX_TICKS / Self::TICKS_PER_MILLISECOND;

    pub const MIN_SECONDS: i64 = Self::MIN_TICKS / Self::TICKS_PER_SECOND;
    pub const MAX_SECONDS: i64 = Self::MAX_TICKS / Self::TICKS_PER_SECOND;

    pub const MIN_MINUTES: i64 = Self::MIN_TICKS / Self::TICKS_PER_MINUTE;
    pub const MAX_MINUTES: i64 = Self::MAX_TICKS / Self::TICKS_PER_MINUTE;

    pub const MIN_HOURS: i64 = Self::MIN_TICKS / Self::TICKS_PER_HOUR;
    pub const MAX_HOURS: i64 = Self::MAX_TICKS / Self::TICKS_PER_HOUR;

    pub const MIN_DAYS: i64 = Self::MIN_TICKS / Self::TICKS_PER_DAY;
    pub const MAX_DAYS: i64 = Self::MAX_TICKS / Self::TICKS_PER_DAY;
}