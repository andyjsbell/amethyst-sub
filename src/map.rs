use rand::Rng;
pub const GRID_SIZE: usize = 32;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, 
    Floor,
}

#[derive(Clone, Default)]
pub struct Map {
    pub rows: usize,
    pub columns: usize,
    pub tiles: Vec<TileType>,
}

/// Grid position, column by row
pub struct GridPosition(pub usize, pub usize);
pub struct GridDimension(pub usize);
pub struct GridRectangle(pub GridPosition, pub GridDimension, pub GridDimension);

impl std::ops::Mul for GridDimension {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
        GridDimension(self.0 * rhs.0)
    }
}

/// Coordinate x, y
pub struct Coordinate(pub f32, pub f32);

impl Map { 
    pub fn grid_to_index(&self, position: GridPosition) -> usize {
        (position.1 * self.columns) + position.0
    }
}

pub fn grid_to_coordinates(position: GridPosition) -> Coordinate {
    Coordinate(
        (position.0 * GRID_SIZE) as f32,
        (position.1 * GRID_SIZE) as f32
    ) 
}

pub fn coordinate_to_grid(coordinate: Coordinate) -> GridPosition {
    GridPosition(
        coordinate.0 as usize / GRID_SIZE,
        coordinate.1 as usize / GRID_SIZE
    )
}

/// Create a simple map width, height and number of random blocks
pub fn create_simple_map(width: usize, height: usize, blocks: usize, player: (usize, usize)) -> Result<Map, String> {
    if width % GRID_SIZE != 0 || height % GRID_SIZE != 0 {
        return Err("Invalid dimensions, we need to be divisable by 32".to_string());
    }

    let columns  = width / GRID_SIZE;
    let rows = height / GRID_SIZE;
    // Add borders, row 0, row N, column 0, column N
    let mut tiles = vec![TileType::Floor; columns * rows];
    // Left and right border
    for i in 0..rows {
        tiles[i * columns] = TileType::Wall;
        tiles[(i * columns) + (columns - 1)] = TileType::Wall;
    }
    // Top and bottom border
    for i in 0..columns {
        tiles[i] = TileType::Wall;
        tiles[(rows * columns) - (i + 1)] = TileType::Wall;
    }

    // Generate random blocks
    let mut rng = rand::thread_rng();
    for _ in 0..blocks {
        let column = rng.gen_range(1..columns - 1);
        let row = rng.gen_range(1..rows - 1);
        if column != player.0 || row != player.1 {
            tiles[(columns * row) + column] = TileType::Wall;
        }
    }
    
    let map = Map {
        rows,
        columns,
        tiles
    };

    Ok(map)
}

pub fn add_room_to_map(room: GridRectangle, map: &mut Map) {
    let rows = room.2.0;
    let cols = room.1.0;
    let pos = room.0;
    for row in 0..rows {
        let start_idx = map.grid_to_index(GridPosition(pos.0, pos.1 + row));
        for idx in start_idx..(start_idx + cols) {
            map.tiles[idx] = TileType::Floor;
        }
    }
}

pub fn create_map(width: usize, height: usize, player: (usize, usize)) -> Result<Map, String> {
    if width % GRID_SIZE != 0 || height % GRID_SIZE != 0 {
        return Err("Invalid dimensions, we need to be divisable by 32".to_string());
    }

    let columns  = width / GRID_SIZE;
    let rows = height / GRID_SIZE;
    let tiles = vec![TileType::Wall; columns * rows];
    let mut map = Map {
        rows,
        columns,
        tiles
    };

    add_room_to_map(
        GridRectangle(
                GridPosition(3,3), 
                GridDimension(5), 
                GridDimension(5)
            ), &mut map);
    
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_to_grid() {
        let coord = Coordinate(100.0, 100.0);
        let grid = coordinate_to_grid(coord);
        assert!(grid.0 == 3 && grid.1 == 3);
    }

    #[test]
    fn test_grid_to_coordinate() {
        let grid = GridPosition(7, 7);
        let coord = grid_to_coordinates(grid);
        assert!(coord.0 == 224.0 && coord.1 == 224.0);
    }
}