use crate::interval_building::{interval::Interval, interval_builder::IntervalBuilder};
use std::ops::Bound;

use comparable_bound::{exc, inc, unb, ComparableBound};

#[test]
fn empty_builder() {
    let interval: Interval<i32> = IntervalBuilder::new().build();

    assert_eq!(interval.get_start(), &unb!());
    assert_eq!(interval.get_end(), &unb!());
}

#[test]
fn inc_inc_builder() {
    let interval = IntervalBuilder::new().from_inc(0).to_inc(1).build();

    assert_eq!(interval.get_start(), &inc!(0));
    assert_eq!(interval.get_end(), &inc!(1));
}

#[test]
fn inc_exc_builder() {
    let interval = IntervalBuilder::new().from_inc(0).to_exc(1).build();

    assert_eq!(interval.get_start(), &inc!(0));
    assert_eq!(interval.get_end(), &exc!(1));
}

#[test]
fn exc_inc_builder() {
    let interval = IntervalBuilder::new().from_exc(0).to_inc(1).build();

    assert_eq!(interval.get_start(), &exc!(0));
    assert_eq!(interval.get_end(), &inc!(1));
}

#[test]
fn exc_exc_builder() {
    let interval = IntervalBuilder::new().from_exc(0).to_exc(1).build();

    assert_eq!(interval.get_start(), &exc!(0));
    assert_eq!(interval.get_end(), &exc!(1));
}
