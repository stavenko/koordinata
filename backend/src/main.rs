use legion::*;
use futures::channel::mpsc::{ unbounded, UnboundedReceiver};
use futures::StreamExt;
use nalgebra as na;
use std::time::SystemTime;
use num::traits::Zero;
mod timer;
mod climate;
mod map;
mod settings;
mod tree;
mod movements;
mod interpolations;

use timer::{
  Timers,
  update_precise_timer,
};
use movements::{ 
  update_positions_system,

};


async fn executor(
  mut channel: UnboundedReceiver<()>, 
  mut world: World,
  mut resources: Resources,
  mut fast: Schedule,
  mut slow: Schedule
) {
  let mut prev_time = SystemTime::now();
  slow.execute(&mut world, &mut resources);
  loop {
    let _msg = tokio::select! {
      _msg = channel.next() => {
      },
      _ = tokio::time::sleep(tokio::time::Duration::from_millis(10)) => {
      }
    };
    fast.execute(&mut world, &mut resources);
    if let Ok(dur) = prev_time.elapsed()  {
      if dur.as_secs() >= 5 { prev_time = SystemTime::now();
        slow.execute(&mut world, &mut resources);
        println!("slow execution time {}", prev_time.elapsed().map(|dur| dur.as_secs_f32()).unwrap_or(0.0));
      }
    }
  }
}





#[tokio::main]
async fn main() {
  let mut w = World::default();
  let mut resources = Resources::default();
  let settings = settings::Settings{
    days_in_season: 2,
    day_duration: 60.0,
    climate_zone: "moderate".into()
  };
  let (_tx, rx) = unbounded();

  let seconds_in_year = settings.seconds_in_year();

  resources.insert(climate::Weather::prepare(&settings.climate_zone));
  resources.insert(settings);
  resources.insert(map::Map::test_square(10., 20.));
  resources.insert(Timers::default());

  use movements::Position;

  tree::place_tree_test(&mut w, Position::zero(), 5.0*seconds_in_year, 10.0);

  let fast_scheduler = Schedule::builder()
    .add_system(update_positions_system())
    .build()
    ;

  let slow_scheduler = Schedule::builder()
    .add_system(timer::timer_update_system())
    .add_system(climate::weather_system())
    .flush()
    .add_system(tree::update_trees_system())
    .add_system(tree::update_offspring_system())
    .add_system(tree::drop_fruits_system())
    .build()
    ;

  executor(rx, w, resources, fast_scheduler, slow_scheduler).await;
}
