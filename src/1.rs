
pub fn random_rust_code() {
    let mut rng = rand::thread_rng();

    let num1 = rng.gen_range(0..=10);
    let num2 = rng.gen_range(0..=10);

    if num1 > num2 {
        return num1;
    } else {
        return num2;
    }
}