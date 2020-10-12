use crate::data::condition::Condition;

pub struct ContainsStringCondition {
    substring: String,
    ignore_case: bool
}

impl ContainsStringCondition {
    pub fn new(substring: String, ignore_case: bool) -> ContainsStringCondition {
        return ContainsStringCondition{substring, ignore_case};
    }
}

impl Condition<String> for ContainsStringCondition {
    fn is_match(&self, data: &String) -> bool {
        if self.ignore_case {
            return data.to_lowercase().contains(&self.substring.to_lowercase());
        } else {
            return data.contains(&self.substring);
        }
    }
}