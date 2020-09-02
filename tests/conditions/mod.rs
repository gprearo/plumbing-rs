#[cfg(test)]
mod always_true;
mod always_false;

pub use always_false::FalseCondition;
pub use always_true::TrueCondition;