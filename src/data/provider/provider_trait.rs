pub trait DataProvider<TData> {
    fn get_data(&self) -> TData;
}

pub trait PathedDataProvider<TData, TPath> {
    fn get_data(&self, path: TPath) -> TData;
}