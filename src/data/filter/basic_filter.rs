use crate::data::filter::condition::Condition;
use crate::data::filter::Filter;
pub struct BasicFilter<T> {
    condition: Box<dyn Condition<T>>
}

impl<T> BasicFilter<T> {
    pub fn new(condition: Box<dyn Condition<T>>) -> BasicFilter<T> {
        return BasicFilter {condition};
    }
}

impl<T> Filter<T> for BasicFilter<T> {
    fn filter(&self, data: Vec<T>) -> Vec<T> {
        let mut filtered = Vec::new();
        for item in data {
            let data = item;
            if self.condition.is_match(&data){
                filtered.push(data);
            }
        }

        return filtered;
    }
}