
use crate::data::condition::Condition;

pub struct OrCondition<T> {
    conditions: Vec<Box<dyn Condition<T>>>
}

impl<'a, T> OrCondition<T> {
    pub fn new(conditions: Vec<Box<dyn Condition<T>>>) -> OrCondition<T> {
        return OrCondition{conditions};
    }

    pub fn boxed_new(conditions: Vec<Box<dyn Condition<T>>>) -> Box<OrCondition<T>> {
        return Box::new(OrCondition::new(conditions));
    }
}

impl<T> Condition<T> for OrCondition<T> {
    fn is_match(&self, data: &T) -> bool {
        return (&self.conditions).into_iter().any(| cond| cond.is_match(data));
    }
}