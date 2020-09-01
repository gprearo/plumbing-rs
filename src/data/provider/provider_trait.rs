pub trait DataProvider<TData> {
    fn get_data() -> TData;
}

pub trait PathedDataProvider<TData, TPath> {
    fn get_data(path: TPath) -> TData;
}