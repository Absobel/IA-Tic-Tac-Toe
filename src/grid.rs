#[derive(Clone, PartialEq)]
pub enum Cell {
    Empty,
    P1,
    P2,
}

#[derive(Clone)]
pub struct Grid {
    width: usize,
    length: usize,
    chars: [char; 3],
    cells: Vec<Cell>,
    win: usize,
}
impl Grid {
    pub fn new(width: usize, length: usize, chars: [char;3], win: usize) -> Grid {
        Grid {
            width: width,
            length: length,
            chars: chars,
            cells: vec![Cell::Empty; width * length],
            win: win,
        }
    }

    pub fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    pub fn to_coord(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&Cell, String> {
        if x < self.width && y < self.length {
            Ok(&self.cells[self.to_index(x, y)])
        } else {
            Err(format!("Out of bounds: {}/{}, {}/{}", x, self.width, y, self.length))
        }
    }
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) -> Result<(), String> {
        if x < self.width && y < self.length {
            self.cells[self.to_index(x,y)] = cell;
            Ok(())
        } else {
            Err(format!("Out of bounds: {}/{}, {}/{}", x, self.width, y, self.length))
        }
    }

    pub fn display(&self) -> Result<(),String> {
        for y in 0..self.length {
            print!("[");
            for x in 0..self.width {
                match self.get(x, y)? {
                    Cell::Empty => print!("{}", self.chars[0]),
                    Cell::P1 => print!("{}", self.chars[1]),
                    Cell::P2 => print!("{}", self.chars[2]),
                }
            }
            println!("]");
        }
        Ok(())
    }
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
    pub fn is_move_win(&self, x: usize, y: usize, cell: Cell) -> Result<bool, String> {
        if x >= self.width || y >= self.length {
            return Err(format!("Out of bounds: {}/{}, {}/{}", x, self.width, y,self.length));
        }

        let resy = (0..self.width).all(|i| self.cells[self.to_index(x, (y + i)%self.length)] == cell);
        let resx = (0..self.length).all(|i| self.cells[self.to_index((x + i)%self.width, y)] == cell);

        Ok(resx || resy)
    }
    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|c| *c != Cell::Empty)
    }
}