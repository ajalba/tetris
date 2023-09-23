use array2d::Array2D;

use crate::game_logic::position::Position;
pub struct Block {
    tiles: Array2D<Position>,
    start_offset: Position,
    offset: Position,
    id: usize,
    rotation_state: usize,
}

impl Block {
    fn rotate_cw(&mut self) {
        self.rotation_state = (self.rotation_state + 1) % self.tiles.num_rows() - 1
    }

    fn rotate_counter_cw(&mut self) {
        if self.rotation_state == 0 {
            self.rotation_state = self.tiles.num_rows() - 1
        } else {
            self.rotation_state -= 1;
        }
    }

    fn move_block(&mut self, rows: usize, columns: usize) {
        self.offset.row = rows;
        self.offset.column = columns;
    }

    fn reset(&mut self) {
        self.rotation_state = 0;
        self.offset.row = self.start_offset.row;
        self.offset.column = self.start_offset.column;  
    }

    fn tile_positions(self) -> Vec<Position> {
        let tiles_as_rows = self.tiles.as_rows();
        return tiles_as_rows[self.rotation_state].clone()
    }
}
