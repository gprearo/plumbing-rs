use crate::data::condition::Condition;

pub struct ContainsStringCondition {
    substring: String
}

impl ContainsStringCondition {
    pub fn new(substring: String) -> ContainsStringCondition {
        return ContainsStringCondition{substring};
    }
}

impl Condition<String> for ContainsStringCondition {
    fn is_match(&self, data: &String) -> bool {
        return data.contains(&self.substring);
    }
}