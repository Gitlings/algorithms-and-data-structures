mod base;
use base::Sorts;

fn is_sorted(vector: &Vec<i32>) -> bool {
    for i in 1..vector.len() {
        if vector[i] < vector[i - 1] {
            return false;
        }
    }

    true
}

#[test]
fn test_insertion() {
    let mut vector = base::insertion::SortedVector::new(vec![-1, 24, 0, 0, -1, 3, 24, 9999]);
    vector.sort();
    is_sorted(vector.get());
}
