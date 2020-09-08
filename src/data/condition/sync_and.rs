
use crate::data::condition::Condition;

pub struct SyncAndCondition<T> {
    conditions: Vec<Box<dyn Condition<T> + Sync>>
}

impl<T> SyncAndCondition<T> {
    pub fn new(conditions: Vec<Box<dyn Condition<T> + Sync>>) -> SyncAndCondition<T> {
        return SyncAndCondition{conditions};
    }

    pub fn boxed_new(conditions: Vec<Box<dyn Condition<T> + Sync>>) -> Box<SyncAndCondition<T>> {
        return Box::new(SyncAndCondition::new(conditions));
    }
}

impl<T> Condition<T> for SyncAndCondition<T> 
    where T: Sync{
    fn is_match(&self, data: &T) -> bool {
        return !(self.conditions.iter().any(| cond| !cond.is_match(data)));
    }
}