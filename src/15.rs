// Exercise 1: Sorting integers by their digits
fn main() {
    let mut numbers = vec![(54, "fizz"), (30, "buzz"), (27, "seven")];
    numbers.sort_by_key(|(value, _)| value);

    for (num, result) in &numbers {
        println!("{} is {}.", num, result);
    }
}
