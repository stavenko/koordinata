use amethyst::{
  input::{InputBundle, StringBindings},
  prelude::*,
  renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
  },
  utils::application_root_dir,
};

struct State;

impl SimpleState for State {}

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());
  let app_root = application_root_dir()?;

  let binding_path = app_root.join("config").join("bindings.ron");
  let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

  let display_config_path = app_root.join("config").join("display.ron");
  let rendering_bundle = RenderingBundle::<DefaultBackend>::new().with_plugin(
    RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]),
  );

  let assets_dir = app_root.join("assets");
  let game_data = GameDataBuilder::default()
    .with_bundle(input_bundle)?
    .with_bundle(rendering_bundle)?;
  let mut game = Application::new(assets_dir, State, game_data)?;

  game.run();

  println!("Hello, world!");
  Ok(())
}
