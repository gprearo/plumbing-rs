use std::iter::FromIterator;
pub trait Filter<TCollection, TData> where TCollection : FromIterator<TData>{
    fn filter(&self, data: TCollection) -> TCollection;
}