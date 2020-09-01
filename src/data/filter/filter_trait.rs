pub trait Filter<TCollection, TData> {
    fn filter(&self, data: TCollection) -> TCollection;
}