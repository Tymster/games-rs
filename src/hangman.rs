use ansi_term::Colour::{Green, Red};
use clearscreen::{self, clear};
use rand::Rng;
#[derive(Debug)]
enum State {
    Correct,
    NotGuessed,
    Incorrect,
}
#[derive(Debug)]
struct Letter {
    character: char,
    state: State,
}
pub fn hangman() {
    let mut alphabet: Vec<Letter> = String::from("abcdefghijklmnopqrstuvwxyz")
        .chars()
        .map(|n| Letter {
            character: n,
            state: State::NotGuessed,
        })
        .collect();
    let mut guesses: i8 = 6;
    let words: Vec<String> = std::fs::read_to_string("/usr/local/bin/games/words.txt")
        .unwrap()
        .split("\n")
        .map(|f| f.to_string().to_lowercase())
        .collect::<Vec<String>>();
    let word: String = words[rand::thread_rng().gen_range(0..words.len())]
        .to_string()
        .to_lowercase();
    let mut output: Vec<char> = vec!['_'; word.len()];
    loop {
        if won(&output) {
            clear().unwrap();
            println!("You won");
            break;
        } else if guesses < 1 {
            clear().unwrap();
            println!("You lost the word was : {}", &word);
            break;
        }
        clear().unwrap();
        print_keyboard(&alphabet);

        println!("\n{}", output.iter().collect::<String>());
        println!("You have {guesses} guesses left");

        let mut state: State = State::Incorrect;
        let mut input: String = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: char = input.trim().chars().next().unwrap();

        if already_guessed(&alphabet, input) {
            println!("You have already guessed the letter : {}", input);
        } else {
            if !word.contains(input) {
                println!("You got it wrong");
                guesses -= 1;
            } else {
                for (index, value) in word.chars().enumerate() {
                    if value == input {
                        state = State::Correct;
                        output[index] = input;
                    }
                }
            }
            if let Some(l) = alphabet
                .iter_mut()
                .filter(|item| item.character == input)
                .next()
            {
                l.state = state;
            }
        }
    }
}

fn print_keyboard(alphabet: &Vec<Letter>) {
    for n in alphabet.iter() {
        match n.state {
            State::Correct => eprint!("{}", Green.paint(n.character.to_string())),
            State::Incorrect => eprint!("{}", Red.paint(n.character.to_string())),
            State::NotGuessed => eprint!("{}", n.character),
        }
    }
}
fn already_guessed(alphabet: &Vec<Letter>, letter: char) -> bool {
    for n in alphabet {
        if n.character == letter {
            return match n.state {
                State::NotGuessed => false,
                _ => true,
            };
        }
    }
    false
}

fn won(output: &Vec<char>) -> bool {
    for n in output {
        if n == &'_' {
            return false;
        }
    }
    true
}
