pub trait Condition<T> {
    fn is_match(&self, data: &T) -> bool;
}