use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let num1 = rng.gen_range(0..10);
    let num2 = rng.gen_range(0..10);
    let result = num1 + num2;
    println!("What is {} plus {}?", num1, num2);
    match result {
        Some(answer) => {
            if answer == 4 {
                println!("Correct! The answer is indeed {}", answer);
            } else {
                println!("Incorrect. The correct answer is {}", answer);
            }
        },
        None => {
            println!("There was an error with the equation");
        }
    }
}
