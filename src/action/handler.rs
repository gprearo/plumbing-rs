pub trait Handler<T> {
    fn handle(&self, data: T);
}