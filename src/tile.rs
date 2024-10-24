use crate::rgb::ImageSize;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl Tile {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        debug_assert!(x1 < x2);
        debug_assert!(y1 < y2);
        Self { x1, y1, x2, y2 }
    }

    pub fn split(&self, x_size: usize, y_size: usize) -> Vec<Tile> {
        let mut tiles = Vec::new();
        for y in (self.y1..self.y2).step_by(y_size) {
            for x in (self.x1..self.x2).step_by(x_size) {
                let x2 = std::cmp::min(self.x2, x + x_size);
                let y2 = std::cmp::min(self.y2, y + y_size);
                let tile = Tile::new(x, y, x2, y2);
                tiles.push(tile);
            }
        }
        tiles
    }

    pub fn width(&self) -> usize {
        self.x2 - self.x1
    }

    pub fn height(&self) -> usize {
        self.y2 - self.y1
    }

    pub fn size(&self) -> ImageSize {
        ImageSize {
            width: self.x2 - self.x1,
            height: self.y2 - self.y1,
        }
    }
}


pub struct TileIterator {
    pub tile: Tile,
    pub x: usize,
    pub y: usize,
}

impl Iterator for TileIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.tile.y2 {
            return None;
        }
        let result = (self.x, self.y);
        self.x += 1;
        if self.x >= self.tile.x2 {
            self.x = self.tile.x1;
            self.y += 1;
        }
        Some(result)
    }
}

impl IntoIterator for Tile {
    type Item = (usize, usize);
    type IntoIter = TileIterator;

    fn into_iter(self) -> Self::IntoIter {
        TileIterator { tile: self, x: self.x1, y: self.y1 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_iterator() {
        let tile = Tile::new(0, 0, 2, 2);
        let mut tile_iter = TileIterator { tile: tile, x: 0, y: 0 };
        assert_eq!(tile_iter.next(), Some((0, 0)));
        assert_eq!(tile_iter.next(), Some((1, 0)));
        assert_eq!(tile_iter.next(), Some((0, 1)));
        assert_eq!(tile_iter.next(), Some((1, 1)));
        assert_eq!(tile_iter.next(), None);

        let tile = Tile::new(0, 0, 3, 3);
        for (x, y) in tile {
            println!("({}, {})", x, y);
        }
    }
}
