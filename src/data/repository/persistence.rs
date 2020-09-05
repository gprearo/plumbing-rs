pub trait Persistence<T> {
    fn save(data: T);

    fn save_collection(data: impl Iterator<Item=T>);
}