use crate::data::condition::Condition;
pub struct VectorContainsCondition<TData> 
    where TData: Eq{
    collection: Vec<TData>
}

impl<TData> VectorContainsCondition<TData> 
    where TData: Eq{
    pub fn new(collection: Vec<TData>) -> VectorContainsCondition<TData>{
        return VectorContainsCondition{collection};
    }
}

impl<TData> Condition<TData> for VectorContainsCondition<TData> 
    where TData: Eq{
        fn is_match(&self, data: &TData) -> bool {
            return self.collection.contains(data);
        }
}
