#[cfg(test)]
use plumbing::data::condition::Condition;

pub struct TrueCondition {}

impl TrueCondition {
    pub fn new() -> TrueCondition{
        return TrueCondition{};
    }

    pub fn boxed_new() -> Box<TrueCondition>{
        return Box::new(TrueCondition::new());
    }
}

impl<T> Condition<T> for TrueCondition {
    fn is_match(&self, _data: &T) -> bool {
        return true;
    }
}