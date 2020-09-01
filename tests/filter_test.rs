
#[cfg(test)]
mod tests {
    use plumbing::data::filter::{Filter, BasicFilter};
    use plumbing::data::filter::condition::Condition;
    use std::cmp::Ord;
    struct GreaterThan<T> {
        value: T
    }

    impl<T> GreaterThan<T> {
        pub fn new(value: T) -> GreaterThan<T>{
            return GreaterThan{value};
        }
    }

    impl<T> Condition<T> for GreaterThan<T> where T: Ord{
        fn is_match(&self, data: &T) -> bool {
            return *data > self.value;
        }
    }

    #[test]
    fn i64_filter_test() {
        let test_vec: Vec<i64> = Vec::from([2, 3, 4, 5, 6, 7]);
        let filter: Box<dyn Filter<Vec<i64>, i64>> = Box::new(BasicFilter::new(Box::new(GreaterThan::new(3))));
        let after_filter = filter.filter(test_vec);
        assert_eq!(after_filter[0], 4);
        assert_eq!(after_filter[1], 5);
        assert_eq!(after_filter[2], 6);
        assert_eq!(after_filter[3], 7);
    }
}