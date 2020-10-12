pub trait Property<TStruct, TProperty> {
    fn get_value(&self, data: &TStruct) -> TProperty;
}
