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

pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Grid position, column by row
#[derive(PartialEq, Clone, Copy)]
pub struct GridDimension(pub usize);
#[derive(PartialEq, Clone, Copy)]
pub struct GridPosition(pub GridDimension, pub GridDimension);
#[derive(PartialEq, Clone, Copy)]
pub struct GridRectangle(pub GridPosition, pub GridDimension, pub GridDimension);

impl GridRectangle {
    pub fn intersect(&self, other:&GridRectangle) -> bool {
        let col = other.0.0;
        let row = other.0.1;

        (col.0 >= self.0.0.0 && col.0 <= self.0.0.0 + self.1.0) &&
        (row.0 >= self.0.1.0 && row.0 <= self.0.1.0 + self.2.0)
    }

    pub fn center(&self) -> GridPosition {
        GridPosition((
            self.0.0 + self.1) / 2.into(), 
            (self.0.1 + self.2) / 2.into()
        )
    } 
}

impl std::ops::Mul for GridDimension {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
        GridDimension(self.0 * rhs.0)
    }
}

impl std::ops::Add for GridDimension {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        GridDimension(self.0 + rhs.0)
    }
}

impl std::ops::Div for GridDimension {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        GridDimension(self.0 / rhs.0)
    }
}

impl From<usize> for GridDimension {
    fn from(item: usize) -> Self {
        GridDimension(item)
    }
}

/// Coordinate x, y
pub struct Coordinate(pub f32, pub f32);

impl Map { 
    pub fn grid_to_index(&self, position: GridPosition) -> usize {
        (position.1.0 * self.columns) + position.0.0
    }
}

pub fn grid_to_coordinates(position: GridPosition) -> Coordinate {
    Coordinate(
        (position.0.0 * GRID_SIZE) as f32,
        (position.1.0 * GRID_SIZE) as f32
    ) 
}

pub fn coordinate_to_grid(coordinate: Coordinate) -> GridPosition {
    GridPosition(
        GridDimension(coordinate.0 as usize / GRID_SIZE),
        GridDimension(coordinate.1 as usize / GRID_SIZE)
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

fn add_room_to_map(room: GridRectangle, map: &mut Map) {
    let rows = room.2.0;
    let cols = room.1.0;
    let pos = room.0;
    for row in 0..rows {
        let start_idx = map.grid_to_index(GridPosition(pos.0, pos.1 + GridDimension(row)));
        for idx in start_idx..(start_idx + cols) {
            map.tiles[idx] = TileType::Floor;
        }
    }
}

fn add_tunnel(orientation: Orientation, position: &GridPosition, length: GridDimension, map: &mut Map) {
    match orientation {
        Orientation::Horizontal => {
            for col in 0..length.0 {
                let idx = map.grid_to_index(GridPosition(position.0 + GridDimension(col), position.1));
                map.tiles[idx] = TileType::Floor;
            }
        },
        Orientation::Vertical => {
            for row in 0..length.0 {
                let idx = map.grid_to_index(GridPosition(position.0, position.1 + GridDimension(row)));
                map.tiles[idx] = TileType::Floor;
            }
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
                GridPosition(GridDimension(3),GridDimension(3)), 
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
        assert!(grid.0 == GridDimension(3) && grid.1 == GridDimension(3));
    }

    #[test]
    fn test_grid_to_coordinate() {
        let grid = GridPosition(GridDimension(7), GridDimension(7));
        let coord = grid_to_coordinates(grid);
        assert!(coord.0 == 224.0 && coord.1 == 224.0);
    }
}