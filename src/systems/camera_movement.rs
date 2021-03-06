use amethyst::{
  core::Transform,
  derive::SystemDesc,
  ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
  input::{InputHandler, StringBindings},
  renderer::Camera,
};

#[derive(SystemDesc)]
pub struct CameraMovementSystem;

impl<'s> System<'s> for CameraMovementSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Camera>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (mut transforms, camera, input): Self::SystemData) {
    let movement_speed = 5.0;
    for (_camera, transform) in (&camera, &mut transforms).join() {
      let horizontal_movement = input.axis_value("horizontal");
      let vertical_movement = input.axis_value("vertical");
      if let Some(horizontal_movement_amount) = horizontal_movement {
        transform.set_translation_x(
          transform.translation().x + horizontal_movement_amount * movement_speed,
        );
      }
      if let Some(vertical_movement_amount) = vertical_movement {
        transform
          .set_translation_y(transform.translation().y + vertical_movement_amount * movement_speed);
      }
    }
  }
}
