
use crate::data::condition::Condition;

pub struct OrCondition<T> {
    conditions: Vec<Box<dyn Condition<T>>>
}

impl<T> OrCondition<T> {
    pub fn new(conditions: Vec<Box<dyn Condition<T>>>) -> OrCondition<T> {
        return OrCondition{conditions};
    }
}

impl<T> Condition<T> for OrCondition<T> {
    fn is_match(&self, data: &T) -> bool {
        for condition in &self.conditions{
            if condition.is_match(data) {
                return true
            }
        }

        return false;
    }
}