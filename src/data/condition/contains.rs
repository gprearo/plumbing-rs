use std::iter::Iterator;
use crate::data::condition::Condition;
pub struct ContainsCondition<'a, TCollection>{
    collection: &'a TCollection
}

impl<'a, TCollection> ContainsCondition<'a, TCollection> {
    pub fn new(collection: &'a TCollection) -> ContainsCondition<TCollection>{
        return ContainsCondition{collection};
    }
}

impl<'a, TCollection, TData> Condition<TData> for ContainsCondition<'a, TCollection> 
    where for<'b> &'b TCollection: IntoIterator<Item=&'b TData> ,
          TData: PartialEq {
        fn is_match(&self, data: &TData) -> bool {
            return self.collection.into_iter().any(| item| *item == *data);
        }
}