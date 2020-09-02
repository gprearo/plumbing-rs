use plumbing::data::condition::Condition;

pub struct FalseCondition {}

impl FalseCondition {
    pub fn new() -> FalseCondition{
        return FalseCondition{};
    }
}

impl<T> Condition<T> for FalseCondition {
    fn is_match(&self, _data: &T) -> bool {
        return false;
    }
}