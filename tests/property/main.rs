use chrono::{Utc, Datelike};
use plumbing::data::property::Property;

pub struct TestData {
    pub name: String,
    pub last_name: String,
    pub age: i32
}

pub struct NameProperty {}

impl NameProperty{
    pub fn new() -> NameProperty {
        return NameProperty{};
    }
}

impl Property<TestData, String> for NameProperty {
    fn get_value<'a>(&self, data: &'a TestData) -> &'a String {
        return &data.name;
    }
}
#[test]
fn string_property_test() {
    //Added chrono, now you will have the exactly Michael Jackson age, whenever you run this test
    let michael_age = Utc::now().year() - 1968;
    let data = TestData {name: String::from("Michael"), last_name: String::from("Jackson"), age: michael_age};
    let name = NameProperty::new();

    assert_eq!(name.get_value(&data), "Michael");
}