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
    let interval1 = Interval::new(exc!(1), exc!(4));
    let interval2 = Interval::new(exc!(2), exc!(3));

    assert_eq!(
        interval1.compare_other(&interval2),
        OverlapOrdering::SuperSet
    );
}

#[test]
fn sub_set() {
    let interval1 = Interval::new(exc!(2), exc!(3));
    let interval2 = Interval::new(exc!(1), exc!(4));

    assert_eq!(interval1.compare_other(&interval2), OverlapOrdering::SubSet);
}

#[test]
fn less() {
    let interval1 = Interval::new(exc!(1), exc!(2));
    let interval2 = Interval::new(exc!(3), exc!(4));

    assert_eq!(interval1.compare_other(&interval2), OverlapOrdering::Less);
}

#[test]
fn overlap_start() {
    let interval1 = Interval::new(exc!(1), exc!(3));
    let interval2 = Interval::new(exc!(2), exc!(4));

    assert_eq!(
        interval1.compare_other(&interval2),
        OverlapOrdering::OverlapStart
    );
}

#[test]
fn equal_start() {
    let interval1 = Interval::new(exc!(1), inc!(2));
    let interval2 = Interval::new(inc!(2), exc!(3));

    assert_eq!(
        interval1.compare_other(&interval2),
        OverlapOrdering::EqualStart
    );
}

#[test]
fn greater() {
    let interval1 = Interval::new(exc!(3), exc!(4));
    let interval2 = Interval::new(exc!(1), exc!(2));

    assert_eq!(
        interval1.compare_other(&interval2),
        OverlapOrdering::Greater
    );
}

#[test]
fn overlap_end() {
    let interval1 = Interval::new(exc!(2), exc!(4));
    let interval2 = Interval::new(exc!(1), exc!(3));

    assert_eq!(
        interval1.compare_other(&interval2),
        OverlapOrdering::OverlapEnd
    );
}

#[test]
fn equal_end() {
    let interval1 = Interval::new(inc!(2), exc!(3));
    let interval2 = Interval::new(exc!(1), inc!(2));

    assert_eq!(
        interval1.compare_other(&interval2),
        OverlapOrdering::EqualEnd
    );
}

#[test]
fn unrecognizable() {
    let interval1: Interval<i32> = Interval::new(unb!(), unb!());
    let interval2: Interval<i32> = Interval::new(unb!(), unb!());

    assert_eq!(
        interval1.compare_other(&interval2),
        OverlapOrdering::Unrecognizable
    );
}
