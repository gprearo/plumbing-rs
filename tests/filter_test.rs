
#[cfg(test)]
mod tests {
    use plumbing::data::filter::{Filter, BasicFilter};
    use plumbing::data::filter::condition::Condition;
    struct GreaterThan<T> {
        value: T
    }

    impl GreaterThan<i64> {
        pub fn new(value: i64) -> GreaterThan<i64>{
            return GreaterThan{value};
        }
    }

    impl Condition<i64> for GreaterThan<i64> {
        fn is_match(&self, data: &i64) -> bool {
            return *data > self.value;
        }
    }

    #[test]
    fn i64_filter_test() {
        let filter: Box<dyn Filter<i64>> = Box::new(BasicFilter::new(Box::new(GreaterThan::new(3))));
        let after_filter = filter.filter(Vec::from([2, 3, 4, 5, 6, 7]));
        assert_eq!(after_filter[0], 4);
        assert_eq!(after_filter[1], 5);
        assert_eq!(after_filter[2], 6);
        assert_eq!(after_filter[3], 7);
    }
}