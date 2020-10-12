use crate::data::property::Property;
use crate::data::condition::Condition;

pub struct PropertyCondition<TStruct, TProperty> {
    property: Box<dyn Property<TStruct, TProperty>>,
    condition: Box<dyn Condition<TProperty>>
}

impl<TStruct, TProperty> PropertyCondition<TStruct, TProperty> {
    pub fn new(property: Box<dyn Property<TStruct, TProperty>>
               , condition: Box<dyn Condition<TProperty>>) -> PropertyCondition<TStruct, TProperty>{
        return PropertyCondition{property, condition};
    }
}

impl<TStruct, TProperty> Condition<TStruct> for PropertyCondition<TStruct, TProperty> {
    fn is_match(&self, data: &TStruct) -> bool {
        let property: &TProperty = &self.property.get_value(data);
        let cond_result =  self.condition.is_match(property);
        return cond_result;
    }
}