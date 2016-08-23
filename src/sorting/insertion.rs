mod base;
use base::*;

fn main() {
    let mut sorted = base::insertion::SortedVector::new(read_vector());
    sorted.sort();
    println!("Sorted array: {:?}", sorted.get());
}
