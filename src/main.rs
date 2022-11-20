mod grid; use grid::{Grid,Cell};
mod player; use player::{Player};
mod launch_options; use launch_options::*;

use std::io;

fn main() {
    println!("\n\nGreetings professor Falken.");
    println!("Shall we play a game?\n");

    let player: Player = Player::new(Cell::P1);
    let ai: Player = Player::new(Cell::P2);

    let mut current_player = &player;
    
    let mut grid = Grid::new(WIDTH, LENGTH, CHARS, WIN);

    println!("");
    grid.display().expect("Could not display the grid");
    println!("");

    loop {
        let (mut x,mut y) = (0,0);
        match current_player.get_cell() {
            Cell::P1 => {
                let stdin = io::stdin();
                println!("Your turn:");
                let mut input = String::new();
                stdin.read_line(&mut input).expect("Could not read the input");
                loop {
                    match input.trim().parse::<usize>() {
                        Ok(n) => {
                            if grid.is_cell_smth(x,y).expect("Could not check if the cell is empty") {
                                println!("This cell is already taken");
                                input = String::new();
                                stdin.read_line(&mut input).expect("Could not read the input");
                            }
                            else {
                                (x,y) = grid.to_coord(n);
                                break;
                            }
                        },
                        Err(_) => {
                            println!("Please enter a number");
                        }
                    }
                }
            },
            Cell::P2 => {
                println!("AI turn:");
                (x,y) = ai.best_move(&player, &grid).expect("Could not find the best move");
                grid.set(x,y,Cell::P2).expect("Could not set the cell");
            },
            _ => panic!("Invalid player"),
        }

        grid.set(x,y,current_player.get_cell()).expect("Could not set the cell");
        if grid.is_move_win(x,y,current_player.get_cell()).expect("Could not check if the move is a win") {
            println!("");
            println!("{:?} wins!", current_player.get_cell());
            break;
        }
        else if grid.is_full() {
            println!("");
            println!("It's a draw!");
            break;
        }
        println!("");
        grid.display().expect("Could not display the grid");
        println!("");

        match current_player.get_cell() {
            Cell::P1 => current_player = &ai,
            Cell::P2 => current_player = &player,
            _ => panic!("Invalid player"),
        }
    }
    
}
