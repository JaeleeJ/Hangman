// requirements - have some of the following: variables, expressions, conditions, loops, functions
// STRETCH CHALLENGE: implemented object oriented techniques - struct and impl

// import libraries
use rand::seq::SliceRandom;
use std::io;

// declare variables of the game
struct HangmanGame {
    word: String,
    max_guesses: i8,
    available_letters: Vec<char>,
    user_guesses: Vec<char>,
    output_string: Vec<char>,
}

// implement structure in new instance
impl HangmanGame {
    fn new(word: &str, max_guesses: i8) -> HangmanGame {
        let available_letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let output_string: Vec<char> = vec!['_'; word.len()];

        HangmanGame {
            word: String::from(word),
            max_guesses,
            available_letters,
            user_guesses: vec![],
            output_string,
        }
    }

    // choose random word from word list in main
    fn generate_random_word(words_list: &Vec<String>) -> String {
        let word = words_list.choose(&mut rand::thread_rng()).unwrap();
        // show the word
        // comment out below to guess the word
        println!("Word: {:?}", word);
        word.clone()
    }

    // display game status - if player is close to winning or losing
    fn display_status(&self) {
        println!("Word: {:?}, Guesses: {:?}, Tries left: {:?}",
            self.output_string, self.user_guesses, self.max_guesses);
    }

    // check user guess
    fn make_guess(&mut self, guess: char) {
        if self.user_guesses.contains(&guess) {
            println!("You guessed that already. Guess another letter.");
            return;
        }

        // change guess to uppercase
        let guess_upper = guess.to_ascii_uppercase();
        let mut correct_guess = false;

        // replace __ with correct guessed letters
        for (x, letter) in self.word.chars().enumerate() {
            if letter.to_ascii_uppercase() == guess_upper {
                self.output_string[x] = letter;
                correct_guess = true;
            }
        }

        if correct_guess {
            self.user_guesses.push(guess);
            println!("Correct guess!");
        } else {
            self.user_guesses.push(guess);
            self.max_guesses -= 1;
            println!("Wrong guess! Tries left: {:?}", self.max_guesses);
        }
    }

    // check if word is guessed or max guesses 
    fn is_game_over(&self) -> bool {
        self.max_guesses == 0 || !self.output_string.contains(&'_')
    }
}

fn main() {
    // list of words
    let words_list = vec![
        "program".to_string(),
        "beekeeper".to_string(),
        "alligator".to_string(),
        "elephant".to_string(),
        "dodgeball".to_string(),
        "mystery".to_string(),
        "zombies".to_string(),
        "espionage".to_string(),
        "autograph".to_string(),
        "knowledge".to_string(),
    ];

    // create random word to guess and instance of game
    let random_word = HangmanGame::generate_random_word(&words_list);
    let mut game = HangmanGame::new(&random_word, 10);

    // starting display
    println!("Welcome to Hangman!");
    game.display_status();

    while !game.is_game_over() {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // read line
        let guess_char = guess.trim().chars().next().unwrap_or(' ');

        // check if input is valid letter
        if guess_char.is_alphabetic() && guess_char.is_ascii_uppercase() {
            game.make_guess(guess_char);
            game.display_status();
        } else {
            println!("Please enter a valid uppercase letter.");
        }
    }

    // check if guess word is correct
    if game.word == game.output_string.iter().collect::<String>() {
        println!("Congratulations! You guessed the word: {:?}", game.word);
    } else {
        println!("Game over! The word was: {:?}", game.word);
    }
}
