use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum BoundProximity {
    Before,  // 指定された値より限りなく少しだけ小さい
    At,      // 指定された値とちょうど同じ
    After,   // 指定された値より限りなく少しだけ大きい
}

impl PartialOrd for BoundProximity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BoundProximity {
    fn cmp(&self, other: &Self) -> Ordering {
        use BoundProximity::*;
        match (self, other) {
            (Before, Before) | (At, At) | (After, After) => Ordering::Equal,
            (Before, _) => Ordering::Less,
            (After, _) => Ordering::Greater,
            (At, Before) => Ordering::Greater,
            (At, After) => Ordering::Less,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundPoint<T> {
    pub value: T,
    pub proximity: BoundProximity,
}

impl<T: PartialOrd> BoundPoint<T> {
    pub fn new_before(value: T) -> Self {
        BoundPoint {
            value,
            proximity: BoundProximity::Before,
        }
    }

    pub fn new_after(value: T) -> Self {
        BoundPoint {
            value,
            proximity: BoundProximity::After,
        }
    }

    pub fn new_at(value: T) -> Self {
        BoundPoint {
            value,
            proximity: BoundProximity::At,
        }
    }
}

impl<T: PartialOrd> PartialOrd for BoundPoint<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Equal) => self.proximity.partial_cmp(&other.proximity),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(BoundProximity::Before, BoundProximity::Before, Ordering::Equal)]
    #[case(BoundProximity::Before, BoundProximity::After, Ordering::Less)]
    #[case(BoundProximity::Before, BoundProximity::At, Ordering::Less)]
    #[case(BoundProximity::After, BoundProximity::Before, Ordering::Greater)]
    #[case(BoundProximity::After, BoundProximity::After, Ordering::Equal)]
    #[case(BoundProximity::After, BoundProximity::At, Ordering::Greater)]
    #[case(BoundProximity::At, BoundProximity::Before, Ordering::Greater)]
    #[case(BoundProximity::At, BoundProximity::After, Ordering::Less)]
    #[case(BoundProximity::At, BoundProximity::At, Ordering::Equal)]
    fn test_bound_proximity_ordering(#[case] value: BoundProximity, #[case] other: BoundProximity, #[case] expected: Ordering) {
        let actual = value.cmp(&other);
        assert_eq!(
            actual, expected,
            "failed: {:?}, {:?} → got {:?}, expected {:?}",
            value, other, actual, expected
        );
    }

    #[rstest]
    #[case(BoundPoint::new_before(1), BoundPoint::new_before(2), Some(Ordering::Less))]
    #[case(BoundPoint::new_before(1), BoundPoint::new_before(0), Some(Ordering::Greater))]
    #[case(BoundPoint::new_before(1), BoundPoint::new_before(2), Some(Ordering::Less))]
    fn test_bound_point_ordering(#[case] value: BoundPoint<i32>, #[case] other: BoundPoint<i32>, #[case] expected: Option<Ordering>) {
        let actual = value.partial_cmp(&other);
        assert_eq!(
            actual, expected,
            "failed: {:?}, {:?} → got {:?}, expected {:?}",
            value, other, actual, expected
        );
    }
}
