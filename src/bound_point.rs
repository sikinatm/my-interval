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

/// A wrapper around an extended bound value, representing
/// a specific endpoint of an interval on an ordered axis.
///
/// `BoundPoint<T>` derives `PartialOrd` and `Ord` through its
/// inner `BoundValue<T>`, allowing it to be compared and sorted
/// alongside other bound points.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct BoundPoint<T>
where
    T: Ord,
{
    pub value: BoundValue<T>,
}

impl<T: Ord> BoundPoint<T> {
    pub fn before(value: T) -> Self {
        Self {
            value: BoundValue::Finite(value, BoundProximity::Before),
        }
    }

    pub fn at(value: T) -> Self {
        Self {
            value: BoundValue::Finite(value, BoundProximity::At),
        }
    }

    pub fn after(value: T) -> Self {
        Self {
            value: BoundValue::Finite(value, BoundProximity::After),
        }
    }

    pub fn neg_infinity() -> Self {
        Self {
            value: BoundValue::NegInfinity,
        }
    }

    pub fn pos_infinity() -> Self {
        Self {
            value: BoundValue::PosInfinity,
        }
    }
}
