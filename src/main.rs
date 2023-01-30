mod hangman;
mod noughts_crosses;
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("hangman") => hangman::hangman(),
        Some("noughts") => noughts_crosses::noughts(),
        _ => println!("Bro idk what your tring to do"),
    }
}
