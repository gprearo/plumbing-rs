#[cfg(test)]
use std::collections::HashSet;
use plumbing::data::filter::{Filter, BasicFilter};
use plumbing::data::condition::{Condition, GreaterThanCondition, OrCondition, AndCondition
                                , VectorContainsCondition, HashSetContainsCondition, SyncAndCondition};
use super::{TrueCondition, FalseCondition};

#[test]
pub fn basic_filter_test() {
    let test_vec: Vec<i64> = Vec::from([2, 3, 4, 5, 6, 7]);
    let filter: Box<dyn Filter<Vec<i64>, i64>> = BasicFilter::boxed_new(Box::new(GreaterThanCondition::new(3)));
    let after_filter = filter.filter(test_vec);
    assert_eq!(after_filter[0], 4);
    assert_eq!(after_filter[1], 5);
    assert_eq!(after_filter[2], 6);
    assert_eq!(after_filter[3], 7);
}
#[test]
pub fn or_condition_test_should_be_true() {
    let cond1: Box<dyn Condition<i64>> = TrueCondition::boxed_new();
    let cond2: Box<dyn Condition<i64>> = FalseCondition::boxed_new();
    let cond_vec = Vec::from([cond1, cond2]);
    let condition = OrCondition::new(cond_vec);

    assert_eq!(condition.is_match(&3), true);
}
#[test]
pub fn or_condition_test_should_be_false() {
    let cond1: Box<dyn Condition<i64>> = FalseCondition::boxed_new();
    let cond2: Box<dyn Condition<i64>> = FalseCondition::boxed_new();
    let cond_vec = Vec::from([cond1, cond2]);
    let condition = OrCondition::new(cond_vec);

    assert_eq!(condition.is_match(&3), false);
}

#[test]
pub fn and_condition_test_should_be_false() {
    let cond1: Box<dyn Condition<i64>> = TrueCondition::boxed_new();
    let cond2: Box<dyn Condition<i64>> = FalseCondition::boxed_new();
    let cond_vec = Vec::from([cond1, cond2]);
    let condition = AndCondition::new(cond_vec);

    assert_eq!(condition.is_match(&3), false);
}
#[test]
pub fn sync_and_condition_test_should_be_true() {
    let cond1: Box<dyn Condition<i64> + Sync> = TrueCondition::boxed_new();
    let cond2: Box<dyn Condition<i64> + Sync> = TrueCondition::boxed_new();
    let cond_vec = Vec::from([cond1, cond2]);
    let condition = SyncAndCondition::new(cond_vec);

    assert_eq!(condition.is_match(&3), true);
}

#[test]
pub fn sync_and_condition_test_should_be_false() {
    let cond1: Box<dyn Condition<i64> + Sync> = TrueCondition::boxed_new();
    let cond2: Box<dyn Condition<i64> + Sync> = FalseCondition::boxed_new();
    let cond_vec = Vec::from([cond1, cond2]);
    let condition = SyncAndCondition::new(cond_vec);

    assert_eq!(condition.is_match(&3), false);
}
#[test]
pub fn and_condition_test_should_be_true() {
    let cond1: Box<dyn Condition<i64>> = TrueCondition::boxed_new();
    let cond2: Box<dyn Condition<i64>> = TrueCondition::boxed_new();
    let cond_vec = Vec::from([cond1, cond2]);
    let condition = AndCondition::new(cond_vec);

    assert_eq!(condition.is_match(&3), true);
}

fn create_iterator_contains_condition() -> Box<dyn Condition<i32>> {
    let cond_vec: Vec<i32> = Vec::from([1, 2, 3, 4]);
    let condition: Box<dyn Condition<i32>> = Box::new(VectorContainsCondition::new(cond_vec));
    return condition;
}

#[test]
pub fn iter_contains_test_should_be_true() {
    let condition = create_iterator_contains_condition();
    assert_eq!(condition.is_match(&3), true);
}

#[test]
pub fn iter_contains_test_should_be_false() {
    let condition = create_iterator_contains_condition();
    assert_eq!(condition.is_match(&12), false);
}

fn create_hash_contains_condition() -> Box<dyn Condition<i32>> {
    let mut cond_vec = HashSet::new();
    cond_vec.insert(1);
    cond_vec.insert(2);
    cond_vec.insert(3);
    cond_vec.insert(4);
    Box::new(HashSetContainsCondition::new(cond_vec))
}

#[test]
pub fn hashset_contains_test_should_be_true() {
    let condition = create_hash_contains_condition();
    assert_eq!(condition.is_match(&3), true);
}

#[test]
pub fn hashset_contains_test_should_be_false() {
    let condition = create_hash_contains_condition();
    assert_eq!(condition.is_match(&20), false);
}
