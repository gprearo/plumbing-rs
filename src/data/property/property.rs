pub trait Property<TStruct, TProperty> {
    fn get_value<'a>(&self, data: &'a TStruct) -> &'a TProperty;
}
