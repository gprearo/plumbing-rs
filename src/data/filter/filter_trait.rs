pub trait Filter<TCollection, TData> {
    fn filter(self: Box<Self>, data: TCollection) -> TCollection;
}