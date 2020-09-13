use crate::data::aggregation::Aggregation;
use crate::data::property::Property;
use std::hash::Hash;
use std::collections::HashMap;

pub struct VectorAggregation<TKey, TElem> {
    key_property: Box<dyn Property<TElem, TKey>>
}

impl<TKey, TElem> VectorAggregation<TKey, TElem> {
    pub fn new(key_property: Box<dyn Property<TElem, TKey>>) -> VectorAggregation<TKey, TElem> {
        return VectorAggregation{key_property};
    }
}

impl<TKey, TElem> Aggregation<Vec<TElem>, TKey, TElem> for VectorAggregation<TKey, TElem> 
    where TKey: Eq + Hash + Copy{
    fn aggregate(&self, collection: Vec<TElem>) -> HashMap<TKey, Vec<TElem>> {
        let mut aggregation: HashMap<TKey, Vec<TElem>> = HashMap::new();
        for elem in collection {
            let elem_key: &TKey  = self.key_property.get_value(&elem);
            if !aggregation.contains_key(elem_key) {
                aggregation.insert(*elem_key, Vec::new());
            }

            if let Some(vector) = aggregation.get_mut(elem_key) {
                vector.push(elem);
            }
        }

        return aggregation;
    }
}