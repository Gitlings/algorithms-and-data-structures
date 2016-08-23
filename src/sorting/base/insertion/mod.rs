include!("../sorting.in");

use super::Sorts;

impl Sorts for SortedVector {
    fn sort(&mut self) {
        for j in 1..self.vector.len() {
            let x = self.vector[j];
            let mut i: i32 = j as i32 - 1;

            while i >= 0 && x < self.vector[i as usize] {
                self.vector[(i + 1) as usize] = self.vector[i as usize];
                i -= 1;
            }
            self.vector[(i + 1) as usize] = x;
        }
    }
}
