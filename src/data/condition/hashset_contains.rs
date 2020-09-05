use std::collections::HashSet;
use std::cmp::Eq;
use std::hash::Hash;
use crate::data::condition::Condition;

pub struct HashSetContainsCondition<'a, TData> 
    where TData: Eq + Hash{
    set: &'a HashSet<TData>
}

impl<'a, TData> HashSetContainsCondition<'a, TData> 
    where TData: Eq + Hash {
    pub fn new(set: &'a HashSet<TData>) -> HashSetContainsCondition<'a, TData> {
        return HashSetContainsCondition{set};
    }
}

impl<'a, TData> Condition<TData> for HashSetContainsCondition<'a, TData> 
    where TData: Eq + Hash {
    fn is_match(&self, data: &TData) -> bool {
        return self.set.contains(data);
    }
}