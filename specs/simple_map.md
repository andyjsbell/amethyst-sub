### Simple Map Specification

We are setting window height and width in RON file.  We will define a tile as either a Wall tile or a Floor tile.

enum TileType {
    Wall,
    Floor,
}

We would create a map of tiles, a tile would have a size of 32 x 32.  
The map would have Wall tiles running its border and a series of random Wall tiles covering the rest of the available space.
The player would be able to move within the map only being able to move to on Floor tiles.
