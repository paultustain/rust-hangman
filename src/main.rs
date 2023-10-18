use hangman::{print_word, read_guess, print_header, validate, choose_word, check_game};

fn main() {
    hangman::print_header();
    let word = choose_word().to_lowercase();
    let mut incomplete: Vec<char> = vec!['_'; word.len()]; 
    let mut lives_left = 10;

    while lives_left > 0 {
        hangman::print_word(&incomplete);
        
        let guess = read_guess();

        if hangman::validate(guess) {
            let guess_used = guess.unwrap().to_lowercase().next().unwrap();
            if word.contains(guess_used) {
                let mut letter_loc = 0;
                for c in word.chars() {
                    if guess_used == c {
                        println!("Correct, good guess!");
                        incomplete[letter_loc] = c;
                    }
                    letter_loc += 1;
                }
            } else {
                lives_left -= 1;
                println!("Incorrect. Lives Left: {}", lives_left);
            }
        } else {
            println!("This is not a letter!");
        }

        if check_game(&incomplete) {
            println!("\n \n \n YOU WIN!");
            break;
        }

    }
}
