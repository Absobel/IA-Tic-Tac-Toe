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

    pub fn move_score(&self, opponent: &Player, grid: &Grid, (x,y): (usize,usize)) -> Result<usize,String> {
        let mut score = 0;
    
        if grid.is_move_win(x,y, self.cell)? {
            score += 100;
        }
        else {
            let mut test_grid = grid.clone();
            test_grid.set(x,y,self.cell)?;
            let empties = test_grid.empties();

        }
        Ok(score)
    }
}