use crate::bound_point::bound_proximity::BoundProximity;

/// Represents an extended bound point on an ordered axis,
/// allowing for negative infinity, finite values with
/// precise inclusion/exclusion semantics, or positive infinity.
///
/// This enum derives `PartialOrd` and `Ord`, with the following ordering:
/// `NegInfinity < Finite(value, proximity) < PosInfinity`.
/// Within `Finite`, comparisons first use the inner `T` value,
/// then the `BoundProximity`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum BoundValue<T>
where
    T: Ord,
{
    /// Represents negative infinity (the lower unbounded limit).
    ///
    /// This is always less than any `Finite` bound or `PosInfinity`.
    NegInfinity,

    /// A finite bound at a specific value, with a proximity marker
    /// indicating whether the interval endpoint is just before,
    /// exactly at, or just after the value.
    ///
    /// - The first element is the value `v` of type `T`.
    /// - The second element is a `BoundProximity` indicating
    ///   whether this bound should be treated as exclusive lower,
    ///   inclusive, or exclusive upper.
    Finite(T, BoundProximity),

    /// Represents positive infinity (the upper unbounded limit).
    ///
    /// This is always greater than any `Finite` bound or `NegInfinity`.
    PosInfinity,
}