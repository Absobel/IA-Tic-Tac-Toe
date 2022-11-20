use super::grid::{Grid,Cell};

pub struct Player {
    cell: Cell,
}
impl Player {
    pub fn new (cell: Cell) -> Player {
        Player {
            cell: cell,
        }
    }
    pub fn get_cell(&self) -> Cell {
        self.cell
    }
    pub fn move_score(&self, opponent: &Player, grid: &Grid, (x,y): (usize,usize)) -> Result<isize,String> {
        let mut score = 0;
    
        if grid.is_move_win(x,y, self.cell)? {
            score += 100;
        }
        else {
            let mut test_grid = grid.clone();
            test_grid.set(x,y,self.cell)?;
            if test_grid.is_full() {
                score += 0;
            }
            else {
                let mut empties = test_grid.empties();
                let opponent_move = opponent.best_move(self, &test_grid)?;
                empties.remove(grid.to_index(opponent_move.0, opponent_move.1));
                if test_grid.is_move_win(opponent_move.0, opponent_move.1, opponent.cell)? {
                    score -= 100;
                }
                else {
                    test_grid.set(opponent_move.0, opponent_move.1, opponent.cell)?;
                    if test_grid.is_full() {
                        score += 0;
                    }
                    else {
                        for (x,y) in empties {
                            score += self.move_score(opponent, &test_grid, (x,y))? as isize;
                        }
                    }
                }
            }
        }
        Ok(score)
    }
    pub fn best_move(&self, opponent: &Player, grid: &Grid) -> Result<(usize,usize), String> {
        let mut scores = vec![];
        let empties = grid.empties();
        for (x,y) in empties {
            scores.push((x,y,self.move_score(opponent, grid, (x,y)).expect("Error in move_score")));
        }
        let mut best_score = 0;
        let mut best_move = (0,0);
        for (x,y,score) in scores {
            if score > best_score {
                best_score = score;
                best_move = (x,y);
            }
        }
        Ok(best_move)
    }
}