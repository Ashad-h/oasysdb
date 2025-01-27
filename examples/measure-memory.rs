use jemalloc_ctl::stats::allocated;
use jemallocator::Jemalloc;
use oasysdb::collection::*;

#[global_allocator]
static ALLOC: Jemalloc = Jemalloc;

fn main() {
    // Modify as needed.
    let len = 10000;
    let dimension = 128;

    // Build the vector collection.
    let records = Record::many_random(dimension, len);
    let config = Config::default();
    Collection::build(config, records).unwrap();

    // Measure the memory usage.
    let memory = allocated::read().unwrap();
    println!("For {} vector records of dimension {}", len, dimension);
    println!("Memory usage: {} bytes", memory);
}
