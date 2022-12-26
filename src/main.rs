mod grid; use grid::{Grid,Cell};
mod player; use player::{Player};
mod launch_options; use launch_options::*;

use std::io;
use std::io::Write;


fn input_coord() -> Result<(usize,usize),String> {                             // Faudrait rendre ça plus propre
    print!("Coordinates : ");
    io::stdout().flush().expect("Could not flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let res_input = input.trim();
    let mut res_input = res_input.split_whitespace();
    let result2_x = res_input.next();
    let result2_y = res_input.next();

    let (x,y);
    let (result_x,result_y);
    match (result2_x,result2_y) {
        (Some(pre2_x), Some(pre2_y)) => {
            result_x = pre2_x.parse::<usize>();
            result_y = pre2_y.parse::<usize>();
            match (result_x,result_y) {
                (Ok(pre_x), Ok(pre_y)) => {
                    x = pre_x;
                    y = pre_y;
                },
                _ => {
                    return Err("These are not coordinates. Format : 'nb_column nb_row'.\n".to_string());
                },
            }
        },
        _ => {
            return Err("These are not coordinates. Format : 'nb_column nb_row'.\n".to_string());
        },
    }
    Ok((x,y))
}

fn main() {
    println!("\n\nGreetings professor Falken.");
    println!("Shall we play a game?\n");

    let (player_cell, ai_cell, mut current_player_cell);

    print!("Choose your character ({} : starts first, or {}): ", CHARS[1], CHARS[2]);
    io::stdout().flush().expect("Could not flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let res_input = input.trim();

    if res_input == CHARS[2].to_string() {
        player_cell = Cell::P2;
        ai_cell = Cell::P1;
        current_player_cell = &ai_cell;
        println!("\nI'll go first then.")
    }
    else {
        player_cell = Cell::P1;
        ai_cell = Cell::P2;
        current_player_cell = &player_cell;
        println!("\nYou'll go first then.")
    }

    let player = Player::new(player_cell);
    let ai = Player::new(ai_cell);

    let mut grid = Grid::new();

    println!();
    grid.display().expect("Could not display the grid");
    println!();

    println!("You will play by entering the coordinates of the cell you want to play in. Format : 'nb_column nb_row'.\n");

    loop {
        let mut mov: Option<(usize, usize)> = None;
        if current_player_cell == &player_cell {
            println!("Your turn.");
            
            if let Ok((x,y)) = input_coord() {
                mov = Some((x,y));
            }
            else {
                println!("\nThese are not coordinates. Format : 'nb_column nb_row'.\n");
                continue;
            }

            let (x,y) = mov.expect("idk, shouldn't happen");
            match grid.set(x, y, player.get_cell()) {
                Ok(_) => {},
                Err(e) => {
                    println!("{}", e);
                    continue;
                },
            }
            current_player_cell = &ai_cell;
        }
        else {
            println!("My turn.");
            if let Ok((x, y,_)) = ai.best_move(&player, &grid, 1){
                mov = Some((x,y));
                grid.set(x, y, ai.get_cell()).expect("Could not set the cell");
            }
            current_player_cell = &player_cell;
        }

        println!();
        grid.display().expect("Could not display the grid");
        println!();

        if grid.is_win_in(mov.expect("idk, shouldn't happen")) {
            if current_player_cell == &ai_cell {
                println!("You won.");
            }
            if current_player_cell == &player_cell {
                println!("I won.");
            }
            break;
        }

        if grid.is_full() {
            println!("It's a draw.");
            break;
        }
    }
    
}
