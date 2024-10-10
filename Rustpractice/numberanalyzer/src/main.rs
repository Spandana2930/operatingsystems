fn main() {
    // Step 1: Create an array of 10 integers
    let numbers = [10, 15, 22, 47, 60, 100, 7, 18, 25, 30];

    // Step 2: Implement the is_even function
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    // Step 3: Use a for loop to iterate through the array
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}:FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}:Fizz", num);
        } else if num % 5 == 0 {
            println!("{}:Buzz", num);
        } else if is_even(num) {
            println!("{}:Even", num);
        } else {
            println!("{}:Odd", num);
        }
    }

    // Step 4: Use a while loop to find the sum of all numbers
    let mut i = 0;
    let mut sum = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("The sum of all numbers is: {}", sum);

    // Step 5: Use a loop to find the largest number in the array
    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is: {}", largest);
}
