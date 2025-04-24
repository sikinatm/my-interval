use crate::bound_point::bound_proximity::BoundProximity;
use crate::bound_point::bound_value::BoundValue;

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
