fn main() {
    let mut rng = rand::thread_rng();
    let number1 = rng.gen_range(1..=10);
    let number2 = rng.gen_range(1..=10);

    println!("What is {} + {}?", number1, number2);
}
