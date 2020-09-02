use crate::data::condition::Condition;

pub struct AndCondition<'a, T> {
    conditions: &'a Vec<Box<dyn Condition<T>>>
}

impl<'a, T> AndCondition<'a, T> {
    pub fn new(conditions: &'a Vec<Box<dyn Condition<T>>>) -> AndCondition<T> {
        return AndCondition{conditions};
    }

    pub fn boxed_new(conditions: &'a Vec<Box<dyn Condition<T>>>) -> Box<AndCondition<T>> {
        return Box::new(AndCondition::new(conditions));
    }
}

impl<'a, T> Condition<T> for AndCondition<'a, T> {
    fn is_match(&self, data: &T) -> bool {
        return !self.conditions.into_iter().any(|cond| !cond.is_match(data));
    }
}