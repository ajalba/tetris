use array2d::Array2D;
pub trait block_logic {
    fn rotate_cw (&self);
    fn rotate_counter_cw(&self);
    fn move_block(&self, rows: usize, columns: usize);
    fn reset(&self);
}
struct Block {
    tiles: Array2D<Position>,
    start_offset: Position,
    offset: Position,
    id: usize,
    rotation_state: usize,
}

// impl Block {
//     pub fn new()
// }
