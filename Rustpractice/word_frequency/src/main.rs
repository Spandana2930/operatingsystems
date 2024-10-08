fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect(); // Split text into words
    let mut word_counts: Vec<(String, usize)> = Vec::new();   // Vector to store word and its frequency

    // Count the frequency of each word
    for word in words {
        let mut found = false;
        // Check if word already exists in the vector
        for (stored_word, count) in word_counts.iter_mut() {
            if stored_word == word {
                *count += 1; // Increment the count
                found = true;
                break;
            }
        }
        // If the word is not found, add it with a count of 1
        if !found {
            word_counts.push((word.to_string(), 1));
        }
    }

    // Find the word with the highest frequency
    let mut max_word = String::new();
    let mut max_count = 0;

    for (word, count) in word_counts {
        if count > max_count {
            max_word = word;
            max_count = count;
        }
    }

    (max_word, max_count) // Return the word and its count
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
