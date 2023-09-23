use array2d::{Array2D, Error};

#[derive(Debug)]
pub struct Grid{
    rows: usize,
    columns: usize,
    grid: Array2D<usize>
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self{
        Grid { rows: rows, columns: columns, grid: Array2D::filled_with(0, rows, columns) }
    }

    pub fn is_inside(&self, row: usize, column: usize) -> bool {
        self.rows > row && self.columns > column
    }

    pub fn is_empty_case(&self, row: usize, column: usize) -> bool {
        let zero :i32 = 0;
        let zero_us = usize::try_from(zero).unwrap();
        return self.is_inside(row, column) && self.grid.get(row, column).unwrap() == &zero_us
    }

    pub fn is_row_full(&self, row_index: usize) -> bool {
        for i in 0..self.columns {
            if self.grid.get(row_index, i).unwrap() == &0 {
                return false
            }
        }

        return true
    }

    pub fn is_row_empty(&self, row_index: usize) -> bool {
        for i in 0..self.columns {
            if self.grid.get(row_index, i).unwrap() != &0 {
                return false
            }
        }

        return true
    }

    pub fn clear_row(&mut self, row_index: usize) {
        for i in 0..self.columns {
            self.grid.get_mut(row_index, i).map(|x| *x = 0);
        }
    }
    
    pub fn move_row(&mut self, row_index: usize, n_deplacements: usize) {
        if n_deplacements != 0 {
            for i in 0..self.columns {
                self.grid.set(row_index + n_deplacements, i, *self.grid.get(row_index, i).unwrap());
                self.grid.get_mut(row_index, i).map(|x| *x = 0);
            }
        }
    }

    pub fn clear_full_rows(&mut self) {
        let mut clear: usize = 0;
        for i in 0..self.rows {
            if self.is_row_empty(self.rows - 1) {
                break;
            }
            else if self.is_row_full(self.rows - i) {
                self.clear_row(self.rows - i);
                clear += 1
            } else {
                self.move_row(self.rows - i, clear)
            }
        }
    }

}
#[cfg(test)]
mod test {
    use crate::game_logic::grid::Grid;
    
    #[test]
    fn grid_creation() {
        let grid = Grid::new(4, 4);
        assert!(grid.grid.as_rows() == vec![vec![0,0,0,0], vec![0,0,0,0], vec![0,0,0,0], vec![0,0,0,0]])
    }

    #[test]
    fn grid_control_element_inside() {
        let grid = Grid::new(4, 4);
        assert_eq!(grid.is_inside(0, 0), true);
        assert_eq!(grid.is_inside(100, 100), false)
    }

    #[test]
    fn grid_control_rows_full_and_clear() {
        let mut grid = Grid::new(4, 4);
        assert_eq!(grid.is_row_empty(2), true);
        for i in 0..4 {
            let _ = grid.grid.set(2, i, 1);
        }
        for i in 0..4 {
            for j in 0..4 {
                print!("{}", grid.grid[(i,j)])
            }
        }
        assert_eq!(grid.is_row_full(2), true);
        grid.clear_row(2);
        assert_eq!(grid.is_row_empty(2), true);
    }


}
