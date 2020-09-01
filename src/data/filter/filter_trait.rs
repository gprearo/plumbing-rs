pub trait Filter<T> {
    fn filter(&self, data: Vec<T>) -> Vec<T>;
}