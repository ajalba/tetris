use array2d::Array2D;

use crate::game_logic::position::Position;

trait BlockLogic {
    fn rotate_cw(&mut self);
    fn rotate_counter_cw(&mut self);
    fn move_block(&mut self, rows: i32, columns: i32);
    fn reset(&mut self);
    fn tile_positions(self) -> Vec<Position>;
}
pub struct Block {
    pub tiles: Array2D<Position>,
    pub start_offset: Position,
    pub offset: Position,
    pub id: usize,
    pub rotation_state: usize,
}

impl BlockLogic for Block {
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

    fn move_block(&mut self, rows: i32, columns: i32) {
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
