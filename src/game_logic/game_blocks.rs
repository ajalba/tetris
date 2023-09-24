use array2d::Array2D;

use crate::game_logic::block::Block;

use super::position::Position;


pub struct IBlock {
    block: Block
}

pub struct OBlock {
    block: Block
}

impl IBlock {
    pub fn new () -> Self {
        let vector_tiles = vec![
            vec![Position{row: 1, column: 0}, Position{row: 1, column: 1}, Position{row: 1, column: 2}, Position{row: 1, column: 3}],
            vec![Position{row: 0, column: 2}, Position{row: 1, column: 2}, Position{row: 2, column: 2}, Position{row: 3, column: 2}],
            vec![Position{row: 2, column: 0}, Position{row: 2, column: 1}, Position{row: 2, column: 2}, Position{row: 2, column: 3}],
            vec![Position{row: 0, column: 1}, Position{row: 1, column: 1}, Position{row: 2, column: 1}, Position{row: 3, column: 1}],
        ];
        let array_tiles = Array2D::from_rows(&vector_tiles).unwrap(); 
        IBlock {
            block: Block {
                tiles: array_tiles,
                start_offset: Position { row: -1, column: 3 },
                offset: Position { row: -1, column: 3 },
                id: 1,
                rotation_state: 0
            }
        }
    }
}

impl OBlock {
    pub fn new () -> Self {
        let vector_tiles = vec![
            vec![Position{row: 0, column: 0}, Position{row: 0, column: 1}, Position{row: 1, column: 0}, Position{row: 1, column: 1}],
            vec![Position{row: 0, column: 0}, Position{row: 0, column: 1}, Position{row: 1, column: 0}, Position{row: 1, column: 1}],
            vec![Position{row: 0, column: 0}, Position{row: 0, column: 1}, Position{row: 1, column: 0}, Position{row: 1, column: 1}],
            vec![Position{row: 0, column: 0}, Position{row: 0, column: 1}, Position{row: 1, column: 0}, Position{row: 1, column: 1}],
        ];
        let array_tiles = Array2D::from_rows(&vector_tiles).unwrap(); 
        OBlock {
            block: Block {
                tiles: array_tiles,
                start_offset: Position { row: 0, column: 4 },
                offset: Position { row: 0, column: 4 },
                id: 4,
                rotation_state: 0
            }
        }
    }
}