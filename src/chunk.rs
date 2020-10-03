use amethyst::ecs::DenseVecStorage;

pub struct ChunkTiles {
  tiles: Vec<Vec<Tile>>,
}

impl Component for ChunkTiles {
    type Storage = DenseVecStorage<Self>;
}

pub struct Tile {
  type: TileType,  
  solid: bool,
}

pub enum TileType {
  GROUND,
  WALL,
}

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

#[derive(SystemDesc)]
pub struct ChunkSystem;

impl<'s> System<'s> for ChunkSystem {
    type SystemData = (
        ReadStorage<'s, ChunkTiles>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
    }
  
}
     

