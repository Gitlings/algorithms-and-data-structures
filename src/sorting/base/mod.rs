use std::io;

pub mod insertion;

pub trait Sorts {
    /**
     * Sorts increasingly.
     */
    fn sort(&mut self);
}

pub fn read_vector() -> Vec<i32> {
    let mut input = String::new();

    println!("Enter an array of integers split via spaces:");
    io::stdin().read_line(&mut input)
        .expect("failed to read line");

    let numbers: Vec<&str> = input.trim().split(' ').collect();
    let mut vector: Vec<i32> = Vec::with_capacity(numbers.len());

    for number in numbers {
        let x = number.parse()
            .expect("Please type an integer!");
        vector.push(x);
    }

    vector
}
