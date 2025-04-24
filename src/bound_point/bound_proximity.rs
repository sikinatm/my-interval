/// Indicates the relative position of a finite bound
/// in the context of an interval.
///
/// This enum is used to distinguish whether a bound
/// should be considered just before, exactly at,
/// or just after a given value. It derives
/// `PartialOrd` and `Ord`, with the natural ordering:
/// `Before < At < After`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoundProximity {
    /// The bound lies immediately _before_ the value.
    ///
    /// Use this for an _exclusive upper bound_,
    /// e.g. `[…, value)`.
    Before,

    /// The bound lies _exactly at_ the value.
    ///
    /// Use this for an _inclusive bound_,
    /// e.g. `[value, …]` or `[…, value]`.
    At,

    /// The bound lies immediately _after_ the value.
    ///
    /// Use this for an _exclusive lower bound_,
    /// e.g. `(value, …)`.
    After,
}