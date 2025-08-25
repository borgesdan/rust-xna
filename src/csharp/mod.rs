pub mod timespan;

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct TimeSpan {
    pub ticks: i64,
}