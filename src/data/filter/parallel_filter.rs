
use crate::data::condition::Condition;
use crate::data::filter::Filter;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub struct ParallelFilter<T> {
    condition: Box<dyn Condition<T> + Sync>
}

impl<T> ParallelFilter<T> {
    pub fn new(condition: Box<dyn Condition<T> + Sync>) -> ParallelFilter<T> {
        return ParallelFilter {condition: condition};
    }

    pub fn boxed_new(condition: Box<dyn Condition<T> + Sync>) -> Box<ParallelFilter<T>> {
        return Box::new(ParallelFilter {condition: condition});
    }
}

impl<T> Filter<Vec<T>, T> for ParallelFilter<T> 
    where T: Sync + Send {
    fn filter(self: Box<Self>, data: Vec<T>) -> Vec<T> {
        return data.into_par_iter().filter(|item| self.condition.is_match(item)).collect();
    }
}