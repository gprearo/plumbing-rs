use std::collections::HashSet;
use std::cmp::Eq;
use std::hash::Hash;
use crate::data::condition::Condition;

pub struct HashSetContainsCondition<TData> 
    where TData: Eq + Hash{
    set: HashSet<TData>
}

impl<TData> HashSetContainsCondition<TData> 
    where TData: Eq + Hash {
    pub fn new(set: HashSet<TData>) -> HashSetContainsCondition<TData> {
        return HashSetContainsCondition{set};
    }
}

impl<TData> Condition<TData> for HashSetContainsCondition<TData> 
    where TData: Eq + Hash {
    fn is_match(&self, data: &TData) -> bool {
        return self.set.contains(data);
    }
}