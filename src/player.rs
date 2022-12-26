use super::grid::{Grid,Cell};
use super::launch_options::*;

pub struct Player {
    cell: Cell,
}
impl Player {
    pub fn new (cell: Cell) -> Player {
        Player {
            cell,
        }
    }
    pub fn get_cell(&self) -> Cell {
        self.cell
    }

    // Renvoie le score d'un coup
    pub fn move_score(
                &self, 
                opponent: &Player, 
                not_real_grid: &mut Grid, 
                (x,y): (usize,usize),
                nb_iter_best_move: usize
            ) -> Result<f32,String> {
        
        let mut score: f32 = 0.0;
        if !not_real_grid.is_full() && nb_iter_best_move < NB_ITER_MAX {
            not_real_grid.set(x, y, self.cell)?;
            if not_real_grid.is_win_in((x,y)) {
                score += 1.0;
            } else if !not_real_grid.is_full() {
                if DEBUG {
                    println!("{nb_iter_best_move}");
                }
                score -= 0.5*opponent.best_move(self, not_real_grid, nb_iter_best_move+1)?.2;
            }
        }
        Ok(score)
    }

    // Renvoie le meilleur coup
    pub fn best_move(
                &self, 
                opponent: &Player, 
                grid: &Grid, 
                nb_iter_best_move: usize
            ) -> Result<(usize,usize,f32), String> {
        
                let empties = grid.empties();
        let mut scores : Vec<f32> = Vec::new();
        for (x,y) in empties.iter() {
            let mut not_real_grid = grid.clone();
            scores.push(self.move_score(opponent, &mut not_real_grid, (*x,*y), nb_iter_best_move)?);
        }
        let mut best_score_idx = 0;
        for i in 1..scores.len() {
            if scores[i] > scores[best_score_idx] {
                best_score_idx = i;
            }
        }
        let var_best_move = empties[best_score_idx];
        Ok((var_best_move.0, var_best_move.1, scores[best_score_idx]))
    }
} 