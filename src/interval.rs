use crate::bound_point::BoundPoint;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntervalType {
    Open,
    StartOpen,
    EndOpen,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval<T>
where
    T: Ord,
{
    start: BoundPoint<T>,
    end: BoundPoint<T>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntervalError {
    StartMustBeMinorThanEnd,
}

    pub fn new(start: T, end: T, interval_type: IntervalType) -> Result<Self, IntervalError> {
impl<T: Ord> Interval<T> {
        Self::validate(&start, &end)?;
        match interval_type {
            IntervalType::Open => Ok(Interval {
                start: BoundPoint::after(start),
                end: BoundPoint::before(end),
            }),
            IntervalType::StartOpen => Ok(Interval {
                start: BoundPoint::after(start),
                end: BoundPoint::at(end),
            }),
            IntervalType::EndOpen => Ok(Interval {
                start: BoundPoint::at(start),
                end: BoundPoint::before(end),
            }),
            IntervalType::Close => Ok(Interval {
                start: BoundPoint::at(start),
                end: BoundPoint::at(end),
            }),
        }
    }

    pub fn since_exclusive(value: T) -> Self {
        Interval {
            start: BoundPoint::after(value),
            end: BoundPoint::pos_infinity(),
        }
    }

    pub fn since_inclusive(value: T) -> Self {
        Interval {
            start: BoundPoint::at(value),
            end: BoundPoint::pos_infinity(),
        }
    }

    pub fn until_exclusive(value: T) -> Self {
        Interval {
            start: BoundPoint::neg_infinity(),
            end: BoundPoint::before(value),
        }
    }

    pub fn until_inclusive(value: T) -> Self {
        Interval {
            start: BoundPoint::neg_infinity(),
            end: BoundPoint::at(value),
        }
    }

    fn validate(start: &T, end: &T) -> Result<(), IntervalError> {
        if start > end {
            Err(IntervalError::StartMustBeMinorThanEnd)
        } else {
            Ok(())
        }
    }

    pub fn contains(&self, value: T) -> bool {
        let bound_point = BoundPoint::at(value);
        self.start <= bound_point && self.end >= bound_point
    }

    pub fn overlaps(&self, other: &Interval<T>) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Interval::new(1, 3, IntervalType::Open).unwrap(), 1,  false)]
    #[case(Interval::new(1, 3, IntervalType::Open).unwrap(), 3,  false)]
    #[case(Interval::new(1, 3, IntervalType::Open).unwrap(), 2,  true)]
    #[case(Interval::new(1, 3, IntervalType::StartOpen).unwrap(), 1,  false)]
    #[case(Interval::new(1, 3, IntervalType::StartOpen).unwrap(), 3,  true)]
    #[case(Interval::new(1, 3, IntervalType::StartOpen).unwrap(), 2,  true)]
    #[case(Interval::new(1, 3, IntervalType::EndOpen).unwrap(), 1,  true)]
    #[case(Interval::new(1, 3, IntervalType::EndOpen).unwrap(), 3,  false)]
    #[case(Interval::new(1, 3, IntervalType::EndOpen).unwrap(), 2,  true)]
    #[case(Interval::new(1, 3, IntervalType::Close).unwrap(), 1,  true)]
    #[case(Interval::new(1, 3, IntervalType::Close).unwrap(), 3,  true)]
    #[case(Interval::new(1, 3, IntervalType::Close).unwrap(), 2,  true)]
    #[case(Interval::new(1, 3, IntervalType::Close).unwrap(), 0,  false)]
    #[case(Interval::new(1, 3, IntervalType::Close).unwrap(), 4,  false)]
    #[case(Interval::from_to(1, 3, IntervalType::Close).unwrap(), 4,  false)]
    // until
    #[case(Interval::until_exclusive(1), 0,  true)]
    #[case(Interval::until_exclusive(1), 1,  false)]
    #[case(Interval::until_exclusive(1), 2,  false)]
    #[case(Interval::until_inclusive(1), 0,  true)]
    #[case(Interval::until_inclusive(1), 1,  true)]
    #[case(Interval::until_inclusive(1), 2,  false)]
    // since
    #[case(Interval::since_exclusive(1), 0,  false)]
    #[case(Interval::since_exclusive(1), 1,  false)]
    #[case(Interval::since_exclusive(1), 2,  true)]
    #[case(Interval::since_inclusive(1), 0,  false)]
    #[case(Interval::since_inclusive(1), 1,  true)]
    #[case(Interval::since_inclusive(1), 2,  true)]
    fn test_contains(#[case] interval: Interval<i32>, #[case] value: i32, #[case] expected: bool) {
        let actual = interval.contains(value);
        assert_eq!(
            actual, expected,
            "failed: {:?}, {} → got {}, expected {}",
            interval, value, expected, actual
        );
    }

    #[rstest]
    #[case(Interval::new(0, 3, IntervalType::Open).unwrap(), Interval::new(0, 3, IntervalType::Open).unwrap(),  true)]
    #[case(Interval::new(0, 3, IntervalType::Open).unwrap(), Interval::new(1, 2, IntervalType::Open).unwrap(),  true)]
    #[case(Interval::new(0, 3, IntervalType::Open).unwrap(), Interval::new(-1, 0, IntervalType::Open).unwrap(),  false)]
    #[case(Interval::new(0, 3, IntervalType::Open).unwrap(), Interval::new(-2, -1, IntervalType::Open).unwrap(),  false)]
    #[case(Interval::new(0, 3, IntervalType::Open).unwrap(), Interval::new(3, 4, IntervalType::Open).unwrap(),  false)]
    #[case(Interval::new(0, 3, IntervalType::Open).unwrap(), Interval::new(4, 5, IntervalType::Open).unwrap(),  false)]
    #[case(Interval::new(0, 3, IntervalType::Open).unwrap(), Interval::new(-1, 2, IntervalType::Open).unwrap(),  true)]
    #[case(Interval::new(0, 3, IntervalType::Open).unwrap(), Interval::new(2, 4, IntervalType::Open).unwrap(),  true)]
    fn test_overlaps(
        #[case] interval: Interval<i32>,
        #[case] other: Interval<i32>,
        #[case] expected: bool,
    ) {
        let actual = interval.overlaps(&other);
        assert_eq!(
            actual, expected,
            "failed: {:?}, {:?} → got {}, expected {}",
            interval, other, expected, actual
        );
    }
}
