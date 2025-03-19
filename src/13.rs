fn main() {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(0..10);
    println!("{}", num);
}
