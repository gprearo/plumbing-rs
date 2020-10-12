use crate::data::merge::{Merge};
use std::iter::Sum;

pub struct SumVectorMerge;

impl<T> Merge<Vec<T>, T> for SumVectorMerge 
    where T: Sum{
    fn merge(&self, data: Vec<T>) -> T {
        return data.into_iter().sum();
    }
}