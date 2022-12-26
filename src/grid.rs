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
                Err("\nCell already occupied.\n".to_string())
            } else {
                self.cells[idx] = cell;
                Ok(())
            }
        } else {
            Err(format!("Out of bounds: {x}/{SIZE}, {y}/{SIZE}\n"))
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
    pub fn is_win_in(&self, (x,y): (usize,usize)) -> bool {
        let cell = self.cells[self.to_index(x, y)];
        if cell == Cell::Empty {
            return false;
        }
        for (dx,dy) in &[(1,0), (0,1), (1,1), (1,-1)] {
            let mut n = 1;
            n += self.nb_alignes_dir((x,y), (*dx,*dy)).expect("Bad direction");
            n += self.nb_alignes_dir((x,y), (-dx,-dy)).expect("Bad direction");
            if n >= NB_ALIGNES {
                return true;
            }
        }
        false
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|c| *c != Cell::Empty)
    }



    // Internal fucntions

    fn voisin(&self, (x,y) : (usize,usize), (dx,dy) : (isize,isize)) -> Option<(usize,usize)> {
        let (x,y) = (x as isize + dx, y as isize + dy);
        if x >= 0 && x < SIZE as isize && y >= 0 && y < SIZE as isize {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }
    fn is_voisin_same(&self, (x,y): (usize,usize), (dx,dy): (isize, isize)) -> Option<bool> {
        if let Some((vx,vy)) = self.voisin((x,y), (dx,dy)) {
            Some(self.cells[self.to_index(vx, vy)] == self.cells[self.to_index(x, y)])
        } else {
            None
        }
    }
    fn nb_alignes_dir(&self, (x,y): (usize, usize), (dx,dy): (isize,isize)) -> Result<usize, String> {
        if !(-1..=1).contains(&dx) || !(-1..=1).contains(&dy) || (dx,dy) == (0,0) {
            return Err("dir invalide. dir ne peut être qu'un vecteur vers l'un des 8 voisins de (x,y)".to_string())
        }
        let mut nb_alignes = 0;
        loop {
            if let Some(is_same) = self.is_voisin_same((x,y), ((nb_alignes+1)*dx, (nb_alignes+1)*dy)) {
                if is_same {
                    nb_alignes += 1;
                } else {
                    return Ok(nb_alignes as usize);
                }
            } else {
                return Ok(nb_alignes as usize);
            }
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