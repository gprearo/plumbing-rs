use std::{collections::HashMap, hash::Hash};
pub trait Aggregation<TCollection, TKey, TElem> 
    where TCollection: IntoIterator<Item=TElem>,
          TKey: Eq + Hash{
    
    fn aggregate(&self, collection: TCollection) -> HashMap<TKey, TCollection> ;
}