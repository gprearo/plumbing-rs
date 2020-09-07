use std::time::Instant;
use rand::Rng;

use plumbing::data::condition::{AndCondition, GreaterThanCondition, LessThanCondition, Condition};
use plumbing::data::filter::{BasicFilter, Filter};

fn main() {
    let mut rng = rand::thread_rng();
    const N: i32 = 1000000;
    let huge_list: Vec<i32> = (0..N).collect();
    let lower_bond = rng.gen_range(0, N-1);
    let upper_bond = rng.gen_range(lower_bond, N);


    let condition: AndCondition<i32> = AndCondition::new(
                                            Vec::from([GreaterThanCondition::boxed_new(lower_bond),
                                            LessThanCondition::boxed_new(upper_bond) as Box<dyn Condition<i32>>]));
    let filter= BasicFilter::boxed_new(Box::new(condition));

    let start = Instant::now();
    let _ = filter.filter(huge_list);
    let elapsed = start.elapsed().as_millis();
    print!("Super abstracted implementation\n");
    print!("Filtered {} i32 in {} milis\n", N, elapsed);
    print!("Lower bond: {} / Upper bond {}\n\n", lower_bond, upper_bond);

    let start = Instant::now();
    let huge_list: Vec<i32> = (0..N).collect();
    let _: Vec<i32> = huge_list.into_iter().filter(| item| *item > lower_bond && *item < upper_bond).collect();
    let elapsed = start.elapsed().as_millis();
    print!("Raw implementation\n");
    print!("Filtered {} i32 in {} milis\n", N, elapsed);
    print!("Lower bond: {} / Upper bond {}\n\n", lower_bond, upper_bond);

}