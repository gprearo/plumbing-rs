use std::time::Instant;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use plumbing::data::condition::{GreaterThanCondition, LessThanCondition, Condition, SyncAndCondition};
use plumbing::data::filter::{Filter, ParallelFilter};

fn main() {
    let mut rng = rand::thread_rng();
    const N: i32 = 1000000;
    let lower_bond = rng.gen_range(0, N-1);
    let upper_bond = rng.gen_range(lower_bond, N);


    let greater_cond: Box<dyn Condition<i32> + Sync> = GreaterThanCondition::boxed_new(lower_bond);
    let less_cond: Box<dyn Condition<i32> + Sync> = LessThanCondition::boxed_new(upper_bond);
    let condition: Box<dyn Condition<i32> + Sync> = SyncAndCondition::boxed_new(
                                            Vec::from([greater_cond, less_cond]));
    let filter= ParallelFilter::boxed_new(condition);

    let huge_list: Vec<i32> = (0..N).collect();
    let start = Instant::now();
    let _ : Vec<i32> = filter.filter(huge_list);
    let elapsed = start.elapsed().as_millis();
    print!("Super abstracted implementation\n");
    print!("Filtered {} i32 in {} milis\n", N, elapsed);
    print!("Lower bond: {} / Upper bond {}\n\n", lower_bond, upper_bond);

    let huge_list: Vec<i32> = (0..N).collect();
    let start = Instant::now();
    let _: Vec<i32> = huge_list.into_par_iter().filter(| item: &i32| *item > lower_bond && *item < upper_bond).collect();
    let elapsed = start.elapsed().as_millis();
    print!("Raw implementation\n");
    print!("Filtered {} i32 in {} milis\n", N, elapsed);
    print!("Lower bond: {} / Upper bond {}\n\n", lower_bond, upper_bond);

}