use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Jane Doe".to_string(), 42);
    scores.insert("John Smith".to_string(), 13);
    for (student, score) in &scores {
        println!("{} has a score of {}", student, score);
    }
}
