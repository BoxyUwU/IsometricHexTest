use vermarine_lib::hexmap::{
    HexMap,
    CHUNK_WIDTH,
    CHUNK_HEIGHT,
    HexChunk,
    Hex,
    Axial,
};

use rand::SeedableRng;
use rand::Rng;
use rand::rngs::StdRng;

use crate::consts::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HexPathNode {
    TopLeft,
    TopRight,
    Right,
    BottomRight,
    BottomLeft,
    Left,

    Goal,
}

impl HexPathNode {
    pub fn from_hexes(start: Hex, end: Hex) -> HexPathNode {
        let start = start.to_axial();
        let end = end.to_axial();

        let (q, r) = (start.q - end.q, start.r - end.r);

        match (q, r) {
            (0, -1) => HexPathNode::TopLeft,
            (1, -1) => HexPathNode::TopRight,
            (1, 0) => HexPathNode::Right,
            (0, 1) => HexPathNode::BottomRight,
            (-1, 1) => HexPathNode::BottomLeft,
            (-1, 0) => HexPathNode::Left,
            (0, 0) => HexPathNode::Goal,
            _ => unreachable!(),
        }
    }

    pub fn to_hexes(&self) -> Hex {
        let (q, r) = match self {
            HexPathNode::TopLeft => (0, -1),
            HexPathNode::TopRight => (1, -1),
            HexPathNode::Right => (1, 0),
            HexPathNode::BottomRight => (0, 1),
            HexPathNode::BottomLeft => (-1, 1),
            HexPathNode::Left => (-1, 0),
            HexPathNode::Goal => (0, 0),
        };

        Axial::new(q, r).to_hex()
    }
}

pub struct Map {
    pub terrain: HexMap<HexTileData>,
    pub dijkstra: HexMap<HexPathNode>,
}

impl Map {
    pub fn new() -> Map {
        let hex_width = 36.;
        let hex_height = 36.;
        let hex_vert_step = 28.;
        let hex_depth_step = 12.;

        let wall_vert_offset = 12.;
        let wall_vert_step = 12.;

        let mut terrain = HexMap::<HexTileData>::new(hex_width, hex_height, hex_vert_step, hex_depth_step, wall_vert_offset, wall_vert_step);

        let mut rand = StdRng::seed_from_u64(10);
        //let mut rand = StdRng::from_entropy();
        let mut chunks = vec![];
        let mut tallest = 0;
        for q in 0..WIDTH {
            for r in 0..HEIGHT {
                let mut tiles = [None; CHUNK_WIDTH * CHUNK_HEIGHT];

                for tile in tiles.iter_mut() {
                    let value = rand.gen_range(0, MAX_FLOOR_HEIGHT as u16 + 1) as u8;
                    *tile = Some(HexTileData::new(value));
                    if value > tallest {
                        tallest = value;
                    }
                }

                chunks.push(HexChunk::new(tiles, q as i32 -1, r as i32 -1));
            }
        }

        terrain.get_height = HexTileData::get_height;

        terrain.tallest = tallest;
        for chunk in chunks.into_iter() {
            terrain.insert_chunk(chunk);
        }

        let mut dijkstra = HexMap::<HexPathNode>::new(hex_width, hex_height, hex_vert_step, hex_depth_step, wall_vert_offset, wall_vert_step); 
        update_dijkstra_hexmap(&terrain, &mut dijkstra, Axial::new(10, 5).to_hex());

        Map {
            terrain,
            dijkstra,
        }
    }

    pub fn get_path(&self, start: Hex) -> Option<Vec<Hex>> {
        let mut path = vec![];

        self.dijkstra.get_tile(start)?;

        let mut current_tile = start;
        loop {
            let path_node = *self.dijkstra.get_tile(current_tile).unwrap();
            
            if path_node == HexPathNode::Goal {
                return Some(path);
            }

            let mut hex = path_node.to_hexes();
            hex += current_tile;
            
            path.push(hex);
            current_tile = hex;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HexTileData {
    pub ground_height: u8,
    pub wall_height: u8,
}

impl HexTileData {
    pub fn new(height: u8) -> HexTileData {
        HexTileData {
            ground_height: height,
            wall_height: height,
        }
    }

    pub fn get_height(&self) -> u8 {
        self.wall_height
    }
}

pub fn update_dijkstra_hexmap(terrain: &HexMap<HexTileData>, dijkstra: &mut HexMap<HexPathNode>, goal: Hex) {
    let mut open_set = vec![goal];
    dijkstra.set_tile(&goal, HexPathNode::Goal);

    while !open_set.is_empty() {
        let length = open_set.len();
        for _ in 0..length {
            let tile = *open_set.first().unwrap();
            let neighbors = tile
                .neighbors()
                .iter()
                .filter(|&&hex| {
                    if terrain.get_tile(hex).is_some() && dijkstra.get_tile(hex).is_none() {
                        let tile_height = terrain.get_tile(tile).unwrap().get_height();
                        let hex_height = terrain.get_tile(hex).unwrap().get_height();
                        
                        let (larger, smaller) = 
                            if hex_height > tile_height {
                                (hex_height, tile_height)
                            } else {
                                (tile_height, hex_height)
                            };
    
                        larger - smaller <= 1
                    } else {
                        false
                    }
                })
                .cloned()
                .collect::<Vec<Hex>>();
        
            for neighbor in neighbors {
                open_set.push(neighbor);
                dijkstra.set_tile(&neighbor, HexPathNode::from_hexes(tile, neighbor));
            }
        
            open_set.remove(0);
        }
    }
}