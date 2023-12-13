use std::{cmp::Ordering, fmt::Display};

use comparable_bound::ComparableBound;

#[derive(Debug, PartialEq, Eq)]
pub enum OverlapOrdering {
    SuperSet,            // (1, 4) in relation to (2, 3)
    SubSet,              // (2, 3) in relation to (1, 4)
    Less,                // (1, 2) in relation to (3, 4)
    OverlapLess,         // (1, 3) in relation to (2, 4)
    OverlapEqualLess,    // (1, 2) in relation to (2, 4)
    Greater,             // (3, 4) in relation to (1, 2)
    OverlapGreater,      // (2, 4) in relation to (1, 3)
    OverlapEqualGreater, // (3, 4) in relation to (1, 3)
    Equal,               // (1, 2) (1, 2)
    Unrecognizable,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Interval<I>
where
    I: PartialOrd,
{
    start: ComparableBound<I>,
    end: ComparableBound<I>,
}

impl<I> Interval<I>
where
    I: PartialEq + PartialOrd,
{
    pub fn new(s: ComparableBound<I>, e: ComparableBound<I>) -> Self {
        Self { start: s, end: e }
    }

    pub fn get_start(&self) -> &ComparableBound<I> {
        &self.start
    }

    pub fn get_end(&self) -> &ComparableBound<I> {
        &self.end
    }

    pub fn compare_other(&self, other: &Self) -> OverlapOrdering {
        match (
            self.start.partial_cmp(&other.end),
            self.end.partial_cmp(&other.start),
        ) {
            (None, None) => OverlapOrdering::Unrecognizable,
            (None, Some(_)) => OverlapOrdering::Unrecognizable,
            (Some(_), None) => OverlapOrdering::Unrecognizable,
            (Some(_), Some(_)) => {
                dbg!("Cross-Comparison is Some");
                self.cross_compare(other)
            }
        }
    }

    fn cross_compare(&self, other: &Self) -> OverlapOrdering {
        match (
            self.start.partial_cmp(&other.end).unwrap(),
            self.end.partial_cmp(&other.start).unwrap(),
        ) {
            (Ordering::Less, Ordering::Less) => OverlapOrdering::Greater,
            (Ordering::Equal, Ordering::Greater) => OverlapOrdering::OverlapEqualLess,
            (Ordering::Equal, Ordering::Equal) => OverlapOrdering::Equal,
            (Ordering::Less, Ordering::Equal) => OverlapOrdering::OverlapEqualGreater,
            (Ordering::Greater, Ordering::Greater) => OverlapOrdering::Less,
            // SuperSet or SubSet
            (Ordering::Less, Ordering::Greater) => self.direct_compare(other),
            _ => OverlapOrdering::Unrecognizable,
        }
    }

    fn direct_compare(&self, other: &Self) -> OverlapOrdering {
        match (
            self.start.partial_cmp(&other.start).unwrap(),
            self.end.partial_cmp(&other.end).unwrap(),
        ) {
            (Ordering::Less, Ordering::Equal) => OverlapOrdering::SuperSet,
            (Ordering::Less, Ordering::Greater) => OverlapOrdering::OverlapGreater,
            (Ordering::Equal, Ordering::Greater) => OverlapOrdering::SuperSet,
            (Ordering::Equal, Ordering::Less) => OverlapOrdering::SuperSet,
            (Ordering::Equal, Ordering::Equal) => OverlapOrdering::Equal,
            (Ordering::Greater, Ordering::Less) => OverlapOrdering::SuperSet,
            (Ordering::Greater, Ordering::Equal) => OverlapOrdering::SubSet,
            (Ordering::Greater, Ordering::Greater) => OverlapOrdering::OverlapLess,
            (Ordering::Less, Ordering::Less) => OverlapOrdering::OverlapGreater,
        }
    }

    pub fn compare_point(&self, other: &I) -> OverlapOrdering {
        self.start_compare(other)
        // match self.start.partial_cmp(&other) {
        //     Some(_) => todo!(),
        //     None => OverlapOrdering::NotPossible,
        // }
    }

    fn start_compare(&self, other: &I) -> OverlapOrdering {
        self.end_compare(other)
    }

    fn end_compare(&self, _: &I) -> OverlapOrdering {
        todo!()
    }
}

impl<I> Display for Interval<I>
where
    I: PartialOrd + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}-{})", self.start, self.end,))
    }
}
