include!("../sorting.in");

use super::Sorts;

impl Sorts for SortedVector {
    fn sort(&mut self) {
        for j in 1..self.vector.len() {
            let x = self.vector[j];
            let mut i = j;

            while i > 0 && x < self.vector[i - 1] {
                self.vector[i] = self.vector[i - 1];
                i -= 1;
            }
            self.vector[i] = x;
        }
    }
}
