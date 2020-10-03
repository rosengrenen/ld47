use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::{
    math::{Point3, Vector3},
    transform::Transform,
  },
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
  tiles::{Tile, TileMap},
};

use noise::{NoiseFn, Perlin};

const SCREEN_WIDTH: f32 = 500.0;
const SCREEN_HEIGHT: f32 = 500.0;

fn initialise_camera(world: &mut World) {
  // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
  let mut transform = Transform::default();
  transform.set_translation_xyz(SCREEN_WIDTH * 0.2, SCREEN_HEIGHT * 0.5, 1.0);

  world
    .create_entity()
    .with(Camera::standard_2d(SCREEN_WIDTH * 2.0, SCREEN_HEIGHT * 2.0))
    .with(transform)
    .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  let loader = world.read_resource::<Loader>();
  let texture_handle = {
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      "texture/wallsground_spritesheet.png",
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };

  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "texture/wallsground_spritesheet.ron",
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}

#[derive(Clone, Default)]
pub struct PerlinTile;
impl Tile for PerlinTile {
  fn sprite(&self, coords: Point3<u32>, world: &World) -> Option<usize> {
    if let Some(perlin) = world.try_fetch::<PerlinResource>() {
      let pre_calculated = &perlin.pre_calculated;
      Some(if pre_calculated[coords.x as usize][coords.y as usize] {
        1
      } else {
        0
      })
    } else {
      panic!("Nooo");
    }
  }
}

#[derive(Default)]
pub struct PerlinResource {
  pub scale: f64,
  pre_calculated: Vec<Vec<bool>>,
}

pub struct PerlinState;

impl SimpleState for PerlinState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    let perlin = Perlin::new();

    let h = 100;
    let w = 100;

    let mut pre_calculated: Vec<Vec<bool>> = Vec::new();

    for x in 0..w {
      let mut v = Vec::new();
      for y in 0..h {
        v.push(if perlin.get([x as f64 / 10.0, y as f64 / 10.0]) < 0.0 {
          true
        } else {
          false
        });
      }

      pre_calculated.push(v);
    }

    world.insert(PerlinResource {
      scale: 5.0,
      pre_calculated,
    });

    let sprite_sheet_handle = load_sprite_sheet(world);

    let map = TileMap::<PerlinTile>::new(
      Vector3::new(w, h, 1),   // The dimensions of the map
      Vector3::new(16, 16, 1), // The dimensions of each tile
      Some(sprite_sheet_handle),
    );
    let transform = Transform::default();

    world.create_entity().with(map).with(transform).build();

    initialise_camera(world);
  }
}
