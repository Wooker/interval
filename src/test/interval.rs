use crate::interval_building::interval::{Interval, OverlapOrdering};
use std::ops::Bound;

use comparable_bound::{exc, inc, unb, ComparableBound};

#[test]
fn unbound() {
    let interval: Interval<i32> = Interval::new(unb!(), unb!());

    assert_eq!(interval.get_start(), &unb!());
    assert_eq!(interval.get_end(), &unb!());
}

#[test]
fn new() {
    let interval = Interval::new(inc!(0), exc!(255));

    assert_eq!(interval.get_start(), &inc!(0));
    assert_eq!(interval.get_end(), &exc!(255));
}

#[test]
fn overlap_equal() {
    let interval1 = Interval::new(inc!(0), exc!(0));
    let interval2 = Interval::new(inc!(0), exc!(0));

    assert_eq!(interval1.compare_other(&interval2), OverlapOrdering::Equal);
}

#[test]
fn overlap_overlap_greater() {
    let interval1 = Interval::new(inc!(0), exc!(3));
    let interval2 = Interval::new(inc!(0), exc!(0));

    assert_eq!(
        interval1.compare_other(&interval2),
        OverlapOrdering::OverlapEqualLess
    );
}
