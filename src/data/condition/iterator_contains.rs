use crate::data::condition::Condition;
pub struct IteratorContainsCondition<'a, TCollection>{
    collection: &'a TCollection
}

impl<'a, TCollection> IteratorContainsCondition<'a, TCollection> {
    pub fn new(collection: &'a TCollection) -> IteratorContainsCondition<TCollection>{
        return IteratorContainsCondition{collection};
    }
}

impl<'a, TCollection, TData> Condition<TData> for IteratorContainsCondition<'a, TCollection> 
    where for<'b> &'b TCollection: IntoIterator<Item=&'b TData> ,
          TData: PartialEq {
        fn is_match(&self, data: &TData) -> bool {
            return self.collection.into_iter().any(| item| *item == *data);
        }
}
