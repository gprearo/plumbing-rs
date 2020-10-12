pub trait Merge<TCollection, T> {
    fn merge(&self, data: TCollection) -> T;
}