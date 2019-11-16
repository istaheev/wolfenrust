pub const TILE_SIZE: u32 = 64;
pub const WALL_HEIGHT: u32 = 64;

pub struct Map {
    tiles: Vec<i32>,
    width: u32,
    height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            tiles: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn new_square(width: u32, height: u32) -> Self {
        let mut map = Self::new(width, height);

        for y in 0..map.height() {
            if y == 0 || y == map.height()-1 {
                for x in 0..map.width() {
                    map.set_tile(x as i32, y as i32, 1);
                }
            } else {
                map.set_tile(0, y as i32, 1);
                map.set_tile((map.width()-1) as i32, y as i32, 1);
            }
        }

        map
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tile(&self, x: i32, y: i32) -> Option<i32> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as u32;
        let y = y as u32;
        if x >= self.width() || y >= self.height() {
            return None;
        }
        let offset = (y * self.width() + x) as usize;
        return Some(self.tiles[offset]);
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: i32) {
        if x < 0 || y < 0 {
            panic!("Map indexes are out bounds");
        }
        let x = x as u32;
        let y = y as u32;
        if x >= self.width() || y >= self.height() {
            panic!("Map indexes are out bounds");
        }
        let offset = (y * self.width() + x) as usize;
        self.tiles[offset] = tile;
    }
}

pub mod bsp_gen {
    use crate::map::Map;

    pub fn gen_map_bsp() -> Map {
        Map::new(64, 64)
    }
}