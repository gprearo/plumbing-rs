use crate::data::condition::Condition;
use std::cmp::Ord;

pub struct GreaterThanCondition<T> {
    value: T
}

impl<T> GreaterThanCondition<T>{
    pub fn new(value: T) -> GreaterThanCondition<T>{
        return GreaterThanCondition{value};
    }

    pub fn boxed_new(value: T) -> Box<GreaterThanCondition<T>>{
        return Box::new(GreaterThanCondition::new(value));
    }
}

impl<T> Condition<T> for GreaterThanCondition<T> 
    where T: Ord {
        fn is_match(&self, data: &T) -> bool{
            return *data > self.value;
        }
}