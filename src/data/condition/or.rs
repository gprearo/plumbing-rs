
use crate::data::condition::Condition;

pub struct OrCondition<'a, T> {
    conditions: &'a Vec<Box<dyn Condition<T>>>
}

impl<'a, T> OrCondition<'a, T> {
    pub fn new(conditions: &'a Vec<Box<dyn Condition<T>>>) -> OrCondition<T> {
        return OrCondition{conditions};
    }

    pub fn boxed_new(conditions: &'a Vec<Box<dyn Condition<T>>>) -> Box<OrCondition<T>> {
        return Box::new(OrCondition::new(conditions));
    }
}

impl<'a, T> Condition<T> for OrCondition<'a, T> {
    fn is_match(&self, data: &T) -> bool {
        return self.conditions.into_iter().any(| cond| cond.is_match(data));
    }
}