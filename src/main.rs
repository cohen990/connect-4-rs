use std::io;

mod overengineered;
mod submission;

fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();

    println!("Welcome to connect 4");

    loop {
        println!("<<Main Menu>>");
        println!(
            "If you'd like to play the normal version, please enter '1'
For the overengineered version enter '2'
Alternatively, if you'd like to exit, please enter '0'"
        );

        input.clear();
        stdin.read_line(input).expect("Error reading from stdio");
        match input.trim() {
            "1" => submission::game_loop::play(),
            "2" => overengineered::game_loop::play(),
            "0" => {
                println!("Thank you for playing!");
                return;
            }
            input => {
                println!("Did not recognise <{}> as an option.", input)
            }
        }
    }
}
