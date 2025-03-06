let mut rng = rand::thread_rng();

// Generate a random integer between 1 and 10
let x: i32 = rng.gen_range(1..=10);

// Generate a random string of length 5
let y: String = rng.sample_iter(&Alphanumeric).take(5).collect();

println!("{} {}", x, y);
