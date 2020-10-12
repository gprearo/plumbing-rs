use plumbing::data::merge::{SumVectorMerge, Merge};

#[test]
pub fn int_sum_merge_test() {
    let int_vector = [1,2,3,4,5,6];
    let sum_merge = SumVectorMerge;

    assert_eq!(sum_merge.merge(int_vector.to_vec()), 21);
}