use crate::data::condition::Condition;

pub struct AndCondition<T> {
    conditions: Vec<Box<dyn Condition<T>>>
}

impl<T> AndCondition<T> {
    pub fn new(conditions: Vec<Box<dyn Condition<T>>>) -> AndCondition<T> {
        return AndCondition{conditions};
    }
}

impl<T> Condition<T> for AndCondition<T> {
    fn is_match(&self, data: &T) -> bool {
        for condition in &self.conditions{
            if !condition.is_match(data) {
                return false
            }
        }

        return true;
    }
}