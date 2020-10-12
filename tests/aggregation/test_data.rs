use plumbing::data::property::Property;
pub struct TestData {
    key1: isize,
    key2: String,
    value: f64
}

impl TestData {
    pub fn new(key1: isize, key2: String, value: f64) -> TestData {
        return TestData {
            key1,
            key2,
            value
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct TestKey {
    key1: isize,
    key2: String
}

pub struct KeyProperty;

impl Property<TestData, TestKey> for KeyProperty {
    fn get_value(&self, data: &TestData) -> TestKey {
        return TestKey{key1: data.key1, key2: data.key2.clone()};
    }
}
