use crate::data::condition::Condition;
use std::cmp::Ord;

pub struct GreaterThanCondition<'a, T> {
    value: &'a T
}

impl<'a, T> GreaterThanCondition<'a, T>{
    pub fn new(value: &'a T) -> GreaterThanCondition<'a, T>{
        return GreaterThanCondition{value};
    }
}

impl<'a, T> Condition<T> for GreaterThanCondition<'a, T> 
    where T: Ord {
        fn is_match(&self, data: &T) -> bool{
            return data > self.value;
        }
}