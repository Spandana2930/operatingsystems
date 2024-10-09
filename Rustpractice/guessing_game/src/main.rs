// Function to check the guess against the secret number
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Correct guess
    } else if guess > secret {
        1 // Guess is too high
    } else {
        -1 // Guess is too low
    }
}

fn main() {
    // Hard-coded secret number
    let secret_number: i32 = 42;
    
    // Variable to track the number of guesses
    let mut guess_count: i32 = 0;

    // Array of predefined guesses to simulate user input
    let guesses: [i32; 3] = [30, 50, 42];

    // Loop through each guess in the array
    for guess in guesses.iter() {
        guess_count += 1; // Increment guess count for each attempt

        // Call the check_guess function
        let result = check_guess(*guess, secret_number); // Dereference guess

        // Use an if-else expression to print the result
        if result == 0 {
            println!("Congratulations! You've guessed the secret number: {}", secret_number);
            break; // Break the loop if the guess was correct
        } else if result == 1 {
            println!("Your guess of {} is too high.", guess);
        } else {
            println!("Your guess of {} is too low.", guess);
        }
    }

    // Print the number of guesses taken
    println!("You took {} guesses to find the secret number.", guess_count);
}
