use super::launch_options::*;

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Cell {
    Empty,
    P1,
    P2,
}


#[derive(Clone)]
pub struct Grid {
    cells: Vec<Cell>,
}
impl Grid {
    // Crée une grille vide
    pub fn new() -> Grid {
        Grid {
            cells: vec![Cell::Empty; SIZE*SIZE],
        }
    }

    // Modifie la cellule aux coordonnées (x,y)
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) -> Result<(), String> {
        let idx = self.to_index(x, y);
        if x < SIZE && y < SIZE {
            if self.cells[idx] != Cell::Empty {
                Err(format!("\nCell already occupied.\n"))
            } else {
                self.cells[idx] = cell;
                Ok(())
            }
        } else {
            Err(format!("Out of bounds: {x}/{SIZE}, {y}/{SIZE}"))
        }
    }

    // Affiche la grille
    pub fn display(&self) -> Result<(),String> {
        print!("    "); (0..SIZE).for_each(|i| print!("{i} "));
        println!();
        for y in 0..SIZE {
            print!("{y} [ ");
            for x in 0..SIZE {
                let cell = self.cells[self.to_index(x, y)];
                match cell {
                    Cell::Empty => print!("{} ", CHARS[0]),
                    Cell::P1 => print!("{} ", CHARS[1]),
                    Cell::P2 => print!("{} ", CHARS[2]),
                }
            }
            println!("]");
        }
        Ok(())
    }

    // Renvoie les coordonnées des cases vides  
    pub fn empties(&self) -> Vec<(usize, usize)> {
        let mut infos = vec![];
        for (i,cell) in self.cells.iter().enumerate() {
            if cell == &Cell::Empty {
                let (x,y) = self.to_coord(i);
                infos.push((x,y));
            }
        }
        infos
    } 

    // Renvoie Some(Cell) si la grille est gagnée, None sinon.
    pub fn is_win(&self) -> Option<Cell> {
        for i in 0..SIZE {
            if let Some(cell) = self.win_ligne(i) {
                return Some(cell);
            }
            if let Some(cell) = self.win_colonne(i) {
                return Some(cell);
            }
        }
        for k in -(SIZE as isize -1)..(SIZE as isize) {
            if let Some(cell) = self.win_diagonale((1, k)) {
                return Some(cell);
            }
            if let Some(cell) = self.win_diagonale((-1, k)) {
                return Some(cell);
            }
        }
        None
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|c| *c != Cell::Empty)
    }



    // Internal fucntions

    // Renvoie Some(Cell) si NB_ALIGNES cases de la ligne i sont identiques, None sinon.
    fn win_ligne(&self, i: usize) -> Option<Cell> {
        for j in 0..SIZE-NB_ALIGNES {
            let mut win = true;
            let cell = self.cells[self.to_index(j, i)];
            if cell != Cell::Empty {
                for k in 1..NB_ALIGNES {
                    win = win && self.cells[self.to_index(j+k, i)] == cell;
                }
                if win {
                    return Some(cell);
                }   
            }
        }
        None
    }
    fn win_colonne(&self, j: usize) -> Option<Cell> {
        for i in 0..SIZE-NB_ALIGNES {
            let mut win = true;
            let cell = self.cells[self.to_index(j, i)];
            if cell != Cell::Empty {
                for k in 1..NB_ALIGNES {
                    win = win && self.cells[self.to_index(j, i+k)] == cell;
                }
                if win {
                    return Some(cell);
                }   
            }
        }
        None
    }
    // On numérote les diagonales de la grille comme suit: (type_diag, num_diag)
    //   0  1 2 3         3 2 1  0
    //  -1 [. . . .]   [. . . .] -1
    //  -2 [. . . .]   [. . . .] -2
    //  -3 [. . . .]   [. . . .] -3
    //     [. . . .]   [. . . .]
    // type_diag = 1   type_diag = -1
    fn win_diagonale(&self, (type_diag, num_diag): (isize, isize)) -> Option<Cell> {
        let size_diag = SIZE - num_diag.abs() as usize;
        if size_diag < NB_ALIGNES {
            return None;
        }
        for i in 0..size_diag-NB_ALIGNES {
            let mut win = true;
            let cell = self.cells[self.to_index(i, i)];
            if cell != Cell::Empty {
                for k in 1..NB_ALIGNES {
                    let (x,y) = if type_diag == 1 && num_diag >= 0 {
                        (i+k+num_diag as usize, i+k)
                    } else if type_diag == 1 && num_diag < 0 {
                        (i+k, i+k-num_diag as usize)
                    } else if type_diag == -1 && num_diag >= 0 {
                        (i+k+num_diag as usize, SIZE-1-i-k)
                    } else {
                        (i+k, SIZE-1-i-k-num_diag as usize)
                    };
                    win = win && self.cells[self.to_index(x, y)] == cell;
                }
                if win {
                    return Some(cell);
                }   
            }
        }
        None
    }

    // Convertit les coordonnées en index dans le vecteur décrivant la grille
    fn to_index(&self, x: usize, y: usize) -> usize {
        y * SIZE + x
    }
    // Convertit l'index dans le vecteur décrivant la grille en coordonnées
    fn to_coord(&self, index: usize) -> (usize, usize) {
        (index % SIZE, index / SIZE)
    }
    
}