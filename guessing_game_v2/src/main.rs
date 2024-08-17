use std::io;

fn main() {
    let mut guess = String::new();
    println!("Guessing Game");
    println!("Input your guess here");

    match io::stdin().read_line(&mut guess) {
        Ok(bytes) => {
            println!("{bytes} bytes read");
            println!("you guessed: {}", guess);
        }
        Err(error) => println!("you caused an error: {error}"),
        
    }
}
