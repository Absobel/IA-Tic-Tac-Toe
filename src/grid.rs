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
        self.win_diagonales()
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|c| *c != Cell::Empty)
    }



    // Internal fucntions

    fn win_ligne(&self, i: usize) -> Option<Cell> {
        if self.cells[i*SIZE] != Cell::Empty && self.cells[i*SIZE] == self.cells[i*SIZE+1] && self.cells[i*SIZE] == self.cells[i*SIZE+2] {
            Some(self.cells[i*SIZE])
        } else {
            None
        }
    }
    fn win_colonne(&self, j: usize) -> Option<Cell> {
        if self.cells[j] != Cell::Empty && self.cells[j] == self.cells[j+SIZE] && self.cells[j] == self.cells[j+2*SIZE] {
            Some(self.cells[j])
        } else {
            None
        }
    }
    fn win_diagonales(&self) -> Option<Cell> {
        if self.cells[0] != Cell::Empty && self.cells[0] == self.cells[4] && self.cells[0] == self.cells[8] {
            Some(self.cells[0])
        } else if self.cells[2] != Cell::Empty && self.cells[2] == self.cells[4] && self.cells[2] == self.cells[6] {
            Some(self.cells[2])
        } else {
            None
        }
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