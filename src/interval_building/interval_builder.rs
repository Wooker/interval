use std::ops::Bound;

use super::interval::Interval;
use comparable_bound::ComparableBound;

pub struct IntervalBuilder<I>
where
    I: PartialOrd + Ord,
{
    from: ComparableBound<I>,
    to: ComparableBound<I>,
}

impl<I> IntervalBuilder<I>
where
    I: PartialOrd + Ord,
{
    pub fn new() -> Self {
        Self {
            from: ComparableBound::new(Bound::Unbounded),
            to: ComparableBound::new(Bound::Unbounded),
        }
    }

    pub fn from_inc(self, from: I) -> Self {
        Self {
            from: ComparableBound::new(Bound::Included(from)),
            to: self.to,
        }
    }

    pub fn from_exc(self, from: I) -> Self {
        Self {
            from: ComparableBound::new(Bound::Excluded(from)),
            to: self.to,
        }
    }

    pub fn to_inc(self, to: I) -> Self {
        Self {
            from: self.from,
            to: ComparableBound::new(Bound::Included(to)),
        }
    }

    pub fn to_exc(self, to: I) -> Self {
        Self {
            from: self.from,
            to: ComparableBound::new(Bound::Excluded(to)),
        }
    }

    pub fn build(self) -> Interval<I> {
        Interval::new(self.from, self.to)
    }
}
