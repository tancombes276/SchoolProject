fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let index_to_remove = rand::thread_rng().gen_range(0..numbers.len());
    numbers.swap_remove(index_to_remove);
}
