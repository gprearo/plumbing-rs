mod test_data;
use test_data::{TestData, KeyProperty, TestKey};
use plumbing::data::aggregation::{Aggregation, VectorAggregation};
use plumbing::data::property::Property;
use std::collections::HashMap;

#[test]
pub fn aggregate_data() {
    let test_data = create_data();
    let key_property = KeyProperty;
    let aggreation = VectorAggregation::new(Box::new(key_property));
    let result = aggreation.aggregate(test_data);

    let right_answer = create_aggregation_result();

    assert_eq!(result.keys().len(), 4);
    for key in result.keys() {
        assert_eq!(result.get(key).unwrap().len(), right_answer.get(key).unwrap().len());
        for row in result.get(key).unwrap() {
            assert!(right_answer.get(key).unwrap().contains(&row));
        }
    }
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

fn create_aggregation_result() -> HashMap<TestKey, Vec<TestData>> {
    let mut result: HashMap<TestKey, Vec<TestData>> = HashMap::new();
    let key_property = KeyProperty;
    let original_data = create_data();

    //Group 1
    result.insert(key_property.get_value(&original_data[0])
                  , Vec::from([original_data[0].clone(), original_data[1].clone()]));
    //Group 2
    result.insert(key_property.get_value(&original_data[2])
                  , Vec::from([original_data[2].clone(), original_data[3].clone()]));
    //Group 3
    result.insert(key_property.get_value(&original_data[4])
                  , Vec::from([original_data[4].clone(), original_data[5].clone()]));
    //Group 4
    result.insert(key_property.get_value(&original_data[6])
                  , Vec::from([original_data[6].clone(), original_data[7].clone()]));

    return result;
}
