use std::io;
use rand::seq::SliceRandom;

fn main() {
    
    let words = vec!["apple",
    "banana",
    "computer",
    "programming",
    "river",
    "mountain",
    "airplane",
    "elephant",
    "guitar",
    "keyboard",
    "happiness",
    "universe",
    "pineapple",
    "chocolate",
    "galaxy",
    "adventure",
    "freedom",
    "rainbow",
    "butterfly",
    "waterfall",
    "sunshine",
    "friendship",
    "dragon",
    "coffee",
    "ocean"];

     // Select a random word
     let mut rng = rand::thread_rng();
     let word = words.choose(&mut rng).unwrap();
 
     // Start letters as underscores
     let mut guessed_word = vec!['_'; word.len()];
 
     // Keep track of guessed letters
     let mut guessed_letters: Vec<char> = Vec::new();
 
     // Set the number of attempts allowed
     let mut attempts = 6;
 
     println!("Welcome to Hangman!");
     println!("You have {} attempts to guess the word.", attempts);
     println!("The word has {} letters.", word.len());
 
     loop {
         println!("Attempts left: {}", attempts);
         println!("Guessed word: {}", guessed_word.iter().collect::<String>());
         println!("Guessed letters: {}", guessed_letters.iter().collect::<String>());
 
         // Prompt the user to enter a letter
         println!("Enter a letter: ");
         let mut input = String::new();
         io::stdin().read_line(&mut input).expect("Failed to read line");
 
         // Convert input to lowercase and get the first letter
         let letter = match input.trim().to_lowercase().chars().next() {
             Some(c) => c,
             None => {
                 println!("Invalid input. Please enter a single letter.");
                 continue;
             }
         };
 
         // Check if the letter has already been guessed
         if guessed_letters.contains(&letter) {
             println!("You have already guessed '{}'.", letter);
             continue;
         }
 
         // Add the guessed letter to the list
         guessed_letters.push(letter);
 
         // Check if the guessed letter is in the word
         let mut found = false;
         for (idx, c) in word.chars().enumerate() {
             if c == letter {
                 guessed_word[idx] = letter;
                 found = true;
             }
         }
 
         // Lower amount of attempts if the letter is not found in the word
         if !found {
             println!("The word doesn't contain '{}'.", letter);
             attempts -= 1;
         }
 
         // Check if the word has been guessed
         if guessed_word.iter().all(|&c| c != '_') {
             println!("Congratulations! You've guessed the word: {}", word);
             break;
         }
 
         // Check if all attempts have been used
         if attempts == 0 {
             println!("Game over! The word was: {}", word);
             break;
         }
     }
}