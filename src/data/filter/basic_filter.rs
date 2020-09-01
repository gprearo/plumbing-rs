use crate::data::condition::Condition;
use crate::data::filter::Filter;
use std::iter::{FromIterator, IntoIterator};
pub struct BasicFilter<'a, T> {
    condition: Box<dyn Condition<T> + 'a>
}

impl<'a, T> BasicFilter<'a, T> {
    pub fn new(condition: impl Condition<T> + 'a) -> BasicFilter<'a, T> {
        return BasicFilter {condition: Box::new(condition)};
    }
}

impl<'a, TCollection, T> Filter<TCollection, T> for BasicFilter<'a, T> 
    where TCollection: FromIterator<T> + IntoIterator<Item = T> {
    fn filter(&self, data: TCollection) -> TCollection {
        return data.into_iter().filter(| item: &T | self.condition.is_match(item) ).collect();
    }
}