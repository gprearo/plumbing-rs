use crate::data::condition::Condition;

pub struct SyncOrCondition<T> {
    conditions: Vec<Box<dyn Condition<T> + Sync>>
}

impl<'a, T> SyncOrCondition<T> {
    pub fn new(conditions: Vec<Box<dyn Condition<T> + Sync>>) -> SyncOrCondition<T> {
        return SyncOrCondition{conditions};
    }

    pub fn boxed_new(conditions: Vec<Box<dyn Condition<T> + Sync>>) -> Box<SyncOrCondition<T>> {
        return Box::new(SyncOrCondition::new(conditions));
    }
}

impl<'a, T> Condition<T> for SyncOrCondition<T> 
    where T: Sync{
    fn is_match(&self, data: &T) -> bool {
        return (&self.conditions).into_iter().any(| cond| cond.is_match(data));
    }
}