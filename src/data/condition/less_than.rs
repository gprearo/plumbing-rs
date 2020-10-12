use crate::data::condition::Condition;
use std::cmp::Ord;

pub struct LessThanCondition<T> {
    value: T
}

impl<T> LessThanCondition<T>{
    pub fn new(value: T) -> LessThanCondition<T>{
        return LessThanCondition{value};
    }

    pub fn boxed_new(value: T) -> Box<LessThanCondition<T>>{
        return Box::new(LessThanCondition::new(value));
    }
}

impl<T> Condition<T> for LessThanCondition<T> 
    where T: Ord {
        fn is_match(&self, data: &T) -> bool{
            return *data < self.value;
        }
}