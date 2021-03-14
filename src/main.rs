use std::io;
mod last_games;

fn main() {
    println!("Welcome to the NBA games/stats application.");
    match last_games::LastGames::get_last_games() {
        Ok(games) => println!("{:#?}", games[0]),
        Err(e) => println!("{}", e),
    }
    let mut option = String::new();
    match io::stdin().read_line(&mut option) {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    }

    println!("Your option {}", option);
}
