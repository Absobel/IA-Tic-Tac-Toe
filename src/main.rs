mod grid; use grid::{Grid,Cell};
mod player; use player::{Player};

use std::io;





fn main() {
    println!("\n\nGreetings professor Falken.");
    println!("Shall we play a game?\n");
    println!("The game is Tic Tac Toe.");
    println!("You will be prompted to enter the coordinates of your move.");
    println!("The coordinates are 0 indexed, so the top left corner is 0, 0.\n");
    println!("You are the X's and I am the O's. Who begins? (you/me)\n");

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Could not read user input");

    let mut current_player = Cell::Empty;
    loop {
        match input.trim() {
            "you" => current_player = Cell::O,
            "me" => current_player = Cell::X,
            _ => println!("I don't understand."),
        }
        if current_player != Cell::Empty {
            break;
        }
    }

    let mut grid = Grid::new(3, 3);

    match current_player {
        Cell::X => {

        }
        Cell::O => {

        }
        _ => {
            println!("Something went wrong.");
        }
    }
    
}
