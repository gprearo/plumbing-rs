use crate::data::condition::Condition;

pub struct EqualsCondition<T> {
    value: T
}

impl<T> EqualsCondition<T> {
    pub fn new(value: T) -> EqualsCondition<T> {
        return EqualsCondition{value};
    }
}

impl<T> Condition<T> for EqualsCondition<T> 
    where T: Eq {
    fn is_match(&self, data: &T) -> bool {
        return self.value == *data;
    }
}