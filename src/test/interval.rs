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
fn equal() {
    let interval1 = Interval::new(inc!(0), exc!(0));
    let interval2 = Interval::new(inc!(0), exc!(0));

    assert_eq!(interval1.compare_other(&interval2), OverlapOrdering::Equal);
}

#[test]
fn super_set() {
    let interval1 = Interval::new(inc!(1), inc!(4));
    let interval2 = Interval::new(inc!(2), inc!(3));

    assert_eq!(
        interval1.compare_other(&interval2),
        OverlapOrdering::SuperSet
    );
}

#[test]
fn sub_set() {
    let interval1 = Interval::new(inc!(2), inc!(3));
    let interval2 = Interval::new(inc!(1), inc!(4));

    assert_eq!(interval1.compare_other(&interval2), OverlapOrdering::SubSet);
}

#[test]
fn less() {
    let interval1 = Interval::new(inc!(1), inc!(2));
    let interval2 = Interval::new(inc!(3), inc!(4));

    assert_eq!(interval1.compare_other(&interval2), OverlapOrdering::Less);
}

#[test]
fn overlap_less() {
    let interval1 = Interval::new(inc!(1), inc!(2));
    let interval2 = Interval::new(inc!(3), inc!(4));

    assert_eq!(interval1.compare_other(&interval2), OverlapOrdering::Less);
}

#[test]
fn overlap_equal_less() {
    let interval1 = Interval::new(inc!(0), exc!(3));
    let interval2 = Interval::new(inc!(0), exc!(0));

    assert_eq!(
        interval1.compare_other(&interval2),
        OverlapOrdering::OverlapEqualLess
    );
}

// Less,                // (1, 2) in relation to (3, 4)
// OverlapLess,         // (1, 3) in relation to (2, 4)
// OverlapEqualLess,    // (1, 2) in relation to (2, 4)
// Greater,             // (3, 4) in relation to (1, 2)
// OverlapGreater,      // (2, 4) in relation to (1, 3)
// OverlapEqualGreater, // (3, 4) in relation to (1, 3)
// Equal,               // (1, 2) (1, 2)
// Unrecognizable,
