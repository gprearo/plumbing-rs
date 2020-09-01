use crate::data::filter::condition::Condition;
use crate::data::filter::Filter;
use std::iter::{FromIterator, IntoIterator};
pub struct BasicFilter<T> {
    condition: Box<dyn Condition<T>>
}

impl<T> BasicFilter<T> {
    pub fn new(condition: Box<dyn Condition<T>>) -> BasicFilter<T> {
        return BasicFilter {condition};
    }
}

impl<TCollection, T> Filter<TCollection, T> for BasicFilter<T> 
    where TCollection: FromIterator<T> + IntoIterator<Item = T> {
    fn filter(&self, data: TCollection) -> TCollection {
        return data.into_iter().filter(| item: &T | self.condition.is_match(item) ).collect();
    }
}