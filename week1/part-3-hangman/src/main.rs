// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);
    
    let len = secret_word_chars.len();
    let mut check: Vec<bool> = vec![false; len];
    let mut guess: [char; 5] = ['0'; 5];
    let mut check_num = 0;

    println!("Welcome to CS110L Hangman!");
    for i in 0..5 {
        print!("The word so far is ");
        for j in 0..check.len() {
            print!("{}",if check[j] {secret_word_chars[j]} else {'-'});
        }
        print!("\nYou have guessed ");
        for j in 0..i {
            print!("{}",guess[j]);
        }
        println!("\nYou have {} guesses left", 5 - i);
        print!("Please guess a letter ");
        std::io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("invalid");
        guess[i] = input.chars().next().unwrap_or('\0');

        let mut st = 0;
        for j in 0..check.len() {
            if check[j]==false && guess[i] == secret_word_chars[j] {
                check[j] = true;
                check_num += 1;
                st = 1;

                if check_num == check.len(){
                    print!("Congratulations you guessed the secret word: ");            
                    for k in 0..check.len() {
                        print!("{}",secret_word_chars[k]);
                    }
                    println!("");
                    return;
                }
            }
        }
        if st == 0 {
            println!("Sorry, that letter is not in the word");
        }

    }
    println!("Sorry, you ran out of guesses!");   
    // Your code here! :)
}
