
fn main() {
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and 10
    let x: i32 = rng.gen_range(1..=10);

    // Ask the user to input their guess for the value of y
    println!("What is your guess for the value of y?");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // Convert the input string to a number using `parse`
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That was not a valid number. Try again.");
            continue;
        }
    };

    // Check if the guess is correct
    if guess == x {
        println!("You are correct! The value of y is {}.", x);
    } else {
        println!("Sorry, that was incorrect. The value of y is {}.", x);
    }
}