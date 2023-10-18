use std::io;
use rand::Rng; 

pub fn print_word(word: &Vec<char>) -> () {
    println!("\n");
    for c in word{
        print!("{} ", c);
    }
    println!("\n");
}


// pub fn check_word(word: &str, guess: &str, mut incomplete: Vec<&str>) -> Vec<&str> {
//     for c in word.chars(){ 
        
//     }
// }

pub fn print_header() -> () {
    println!(r"


 __    __     ____     __      __     ______    ____      ____     ____     __      __   __
|  |  |  |  /  __  \  |   \   |  |  /  _____|  |    \    /    |  /  __  \  |   \   |  | |  |
|  |__|  | |  |__|  | |    \  |  | |  |  ____  |     \  /     | |  |__|  | |    \  |  | |  |
|   __   | |   __   | |  |\ \ |  | |  | |__  | |  |\  \/  /|  | |   __   | |  |\ \ |  | |__|
|  |  |  | |  |  |  | |  | \ \|  | |  \___/  | |  | \    / |  | |  |  |  | |  | \ \|  |  __
|__|  |__| |__|  |__| |__|  \____|  \_______/  |__|  \__/  |__| |__|  |__| |__|  \____| |__|


Welcome to hangman. Time to guess a word. Select a letter and the word will populate if it is correct.



");
}

pub fn read_guess() -> Option<char> {
    let mut guess = String::new();
    println!("Please guess a letter.\n");
    io::stdin().read_line(&mut guess).expect("Failed to read");
    guess.trim().chars().nth(0)

}

pub fn validate(letter: Option<char>) -> bool {
    match letter {
        Some(guess) => {
            if guess.is_alphabetic() { true }
            else { false }
        }
        None => { false }
    }
}

pub fn choose_word() -> String {
    const WORDS: [&str; 6] = ["hey", "word", "fancy", "finals", "classic", "lingering"];
    let num = rand::thread_rng().gen_range(0..WORDS.len()); 
    let word = WORDS[num];
    word.to_string()

}

pub fn check_game(incomplete: &Vec<char>) -> bool {
    if incomplete.contains(&'_') {
        false
    } else {
        true 
    }
}