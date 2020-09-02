use crate::data::condition::Condition;
use std::cmp::Ord;

pub struct LessThanCondition<'a, T> {
    value: &'a T
}

impl<'a, T> LessThanCondition<'a, T>{
    pub fn new(value: &'a T) -> LessThanCondition<'a, T>{
        return LessThanCondition{value};
    }

    pub fn boxed_new(value: &'a T) -> Box<LessThanCondition<'a, T>>{
        return Box::new(LessThanCondition::new(value));
    }
}

impl<'a, T> Condition<T> for LessThanCondition<'a, T> 
    where T: Ord {
        fn is_match(&self, data: &T) -> bool{
            return data < self.value;
        }
}