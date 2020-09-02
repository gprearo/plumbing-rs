#[cfg(test)]
use plumbing::data::filter::{Filter, BasicFilter};
use plumbing::data::condition::{Condition, GreaterThanCondition, OrCondition, AndCondition};

mod conditions;
use conditions::{TrueCondition, FalseCondition};

#[test]
fn i64_filter_test() {
    let test_vec: Vec<i64> = Vec::from([2, 3, 4, 5, 6, 7]);
    let filter: Box<dyn Filter<Vec<i64>, i64>> = Box::new(BasicFilter::new(GreaterThanCondition::new(&3)));
    let after_filter = filter.filter(test_vec);
    assert_eq!(after_filter[0], 4);
    assert_eq!(after_filter[1], 5);
    assert_eq!(after_filter[2], 6);
    assert_eq!(after_filter[3], 7);
}
#[test]
fn or_condition_test_should_be_true() {
    let cond1: Box<dyn Condition<i64>> = Box::new(TrueCondition::new());
    let cond2: Box<dyn Condition<i64>> = Box::new(FalseCondition::new());
    let cond_vec = Vec::from([cond1, cond2]);
    let condition = OrCondition::new(&cond_vec);

    assert_eq!(condition.is_match(&3), true);
}
#[test]
fn or_condition_test_should_be_false() {
    let cond1: Box<dyn Condition<i64>> = Box::new(FalseCondition::new());
    let cond2: Box<dyn Condition<i64>> = Box::new(FalseCondition::new());
    let cond_vec = Vec::from([cond1, cond2]);
    let condition = OrCondition::new(&cond_vec);

    assert_eq!(condition.is_match(&3), false);
}

#[test]
fn and_condition_test_should_be_false() {
    let cond1: Box<dyn Condition<i64>> = Box::new(TrueCondition::new());
    let cond2: Box<dyn Condition<i64>> = Box::new(FalseCondition::new());
    let cond_vec = Vec::from([cond1, cond2]);
    let condition = AndCondition::new(&cond_vec);

    assert_eq!(condition.is_match(&3), false);
}
#[test]
fn and_condition_test_should_be_true() {
    let cond1: Box<dyn Condition<i64>> = Box::new(TrueCondition::new());
    let cond2: Box<dyn Condition<i64>> = Box::new(TrueCondition::new());
    let cond_vec = Vec::from([cond1, cond2]);
    let condition = AndCondition::new(&cond_vec);

    assert_eq!(condition.is_match(&3), true);
}