use crate::data::condition::Condition;
use crate::data::filter::Filter;
use std::iter::{FromIterator, IntoIterator};

pub struct BasicFilter<T> {
    condition: Box<dyn Condition<T>>
}

impl<'a, T> BasicFilter<T> {
    pub fn new(condition: Box<dyn Condition<T>>) -> BasicFilter<T> {
        return BasicFilter {condition: condition};
    }

    pub fn boxed_new(condition: Box<dyn Condition<T>>) -> Box<BasicFilter<T>> {
        return Box::new(BasicFilter {condition: condition});
    }
}

impl<TCollection, T> Filter<TCollection, T> for BasicFilter<T> 
    where TCollection: FromIterator<T> + IntoIterator<Item=T> {
    fn filter(self: Box<Self>, data: TCollection) -> TCollection {
        return data.into_iter().filter(| item | self.condition.is_match(item)).collect();
    }
}