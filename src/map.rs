use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    return ((y * SCREEN_WIDTH) + x) as usize;
}

impl Map {
    pub fn new() -> Self {
        return Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        };
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        return point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT;
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        return self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor;
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            return None;
        } else {
            return Some(map_idx(point.x, point.y));
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        // if self.in_bounds(destination) {
        if self.can_enter_tile(destination) {
            let idx = self.point2d_to_index(destination);
            return Some(idx);
        } else {
            None
        }
    }
    // else {
    // None
    // }
    // }
}

impl Algorithm2D for Map {
    fn point2d_to_index(&self, pt: Point) -> usize {
        let bounds = self.dimensions();
        ((pt.y * bounds.x) + pt.x)
            .try_into()
            .expect("Not a valid usize. Did something go negative?")
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        let bounds = self.dimensions();
        let w: usize = bounds
            .x
            .try_into()
            .expect("Not a valid usize. Did something go negative?");
        Point::new(idx % w, idx / w)
    }

    fn dimensions(&self) -> Point {
        return Point::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    }

    fn in_bounds(&self, pos: Point) -> bool {
        return self.in_bounds(pos);
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exists = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exists.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exists.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exists.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exists.push((idx, 1.0))
        }

        return exists;
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        return DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), 
        self.index_to_point2d(idx2));
    }

    fn is_opaque(&self, idx: usize) -> bool {
        return self.tiles[idx] != TileType::Floor;
    }

    
}
