mod test_data;
use test_data::{TestData, KeyProperty};
use plumbing::data::aggregation::{Aggregation, VectorAggregation};

#[test]
pub fn aggregate_data() {
    let test_data = create_data();
    let key_property = KeyProperty;
    let aggreation = VectorAggregation::new(Box::new(key_property));
    let result = aggreation.aggregate(test_data);

    assert_eq!(result.keys().len(), 4);
}

fn create_data() -> Vec<TestData> {
    let mut data = Vec::with_capacity(8);
    data.push(TestData::new(1, String::from("A"), 1.0));
    data.push(TestData::new(1, String::from("A"), 2.0));
    data.push(TestData::new(2, String::from("A"), 3.0));
    data.push(TestData::new(2, String::from("A"), 4.0));
    data.push(TestData::new(1, String::from("B"), 5.0));
    data.push(TestData::new(1, String::from("B"), 6.0));
    data.push(TestData::new(1, String::from("C"), 7.0));
    data.push(TestData::new(1, String::from("C"), 8.0));
    return data;
}