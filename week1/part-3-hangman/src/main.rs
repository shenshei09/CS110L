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
//use std::arch::x86_64::has_cpuid;
use std::collections::HashSet;
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

fn print_word_so_far(word_so_far: &Vec<char>) {
    print!("The word so far is ");
    for i in word_so_far.iter() {
        print!("{}", i);
    }
    println!();
}

fn print_has_guessed(has_guessed: &Vec<char>) {
    print!("You have guessed the following letters: ");
    for i in has_guessed.iter() {
        print!("{}", i);
    }
    println!();
}

fn print_guesses_left(guesses: &u32) {
    println!("You have {} guesses left", guesses);
}

fn guess_char() -> char {
    print!("Please guess a letter: ");

    let mut guess = String::new();
    io::stdout()
        .flush()
        .expect("Error flushing stdout.");
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");

    let guess: Vec<char> = guess.chars().collect();
    return *guess.first().unwrap();
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("random word: {:?}", secret_word);
    println!("random word: {:?}", secret_word_chars);

    println!("Welcome to CS110L Hangman!");

    let mut word_so_far = Vec::new();
    let mut has_guessed = Vec::new();
    let mut not_guessed = HashSet::new();
    let mut guesses = NUM_INCORRECT_GUESSES;

    for _i in 0..secret_word_chars.len() {
        word_so_far.push('-');
    }

    for i in secret_word_chars.iter() {
        not_guessed.insert(*i);
    }

    loop {
        print_word_so_far(&word_so_far);
        print_has_guessed(&has_guessed);
        print_guesses_left(&guesses);

        let ch = guess_char();

        if !has_guessed.contains(&ch) {
            has_guessed.push(ch);
            if not_guessed.contains(&ch) {
                not_guessed.remove(&ch);
                for i in 0..secret_word_chars.len() {
                    if secret_word_chars[i] == ch {
                        word_so_far[i] = ch;
                    }
                }
            } else {
                guesses -= 1;
                println!("Sorry, that letter is not in the word");
            }
        }
        else{
            println!("Sorry, you have guessed this letter");
        }
        
        println!();
        
        if not_guessed.is_empty() {
            println!("Congratulations you guessed the secret word: {}!", secret_word);
            break;
        }
        else if guesses == 0{
            println!("Sorry, you ran out of guesses!");
            break;
        }
    }
}
