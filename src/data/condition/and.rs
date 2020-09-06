use crate::data::condition::Condition;

pub struct AndCondition<T> {
    conditions: Vec<Box<dyn Condition<T>>>
}

impl<T> AndCondition<T> {
    pub fn new(conditions: Vec<Box<dyn Condition<T>>>) -> AndCondition<T> {
        return AndCondition{conditions};
    }

    pub fn boxed_new(conditions: Vec<Box<dyn Condition<T>>>) -> Box<AndCondition<T>> {
        return Box::new(AndCondition::new(conditions));
    }

    fn get_conditions(&self) -> &Vec<Box<dyn Condition<T>>> {
        return &self.conditions;
    }
}

impl<T> Condition<T> for AndCondition<T> {
    fn is_match(&self, data: &T) -> bool {
        return !self.get_conditions().into_iter().any(|cond: &Box<dyn Condition<T>>| !cond.is_match(data));
    }
}