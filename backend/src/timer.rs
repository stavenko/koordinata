use legion::*;
use std::time::SystemTime;
use super::settings::Settings;


pub struct Time {
  pub elapsed_seconds: f32,
  last_timestamp: SystemTime
}

pub enum Season {
  Autumn,
  Summer,
  Winter,
  Spring,
}

pub struct Timers {
  pub precize: Time,
  pub long: Time,
  pub initial: SystemTime,
  pub day_of_year: u16,
  pub day_of_season: u16,
  pub current_season: Season,
  pub time_of_day: f32,
  pub time_of_season: f32,
  pub time_of_year: f32,

}

impl Timers {
  pub fn update_fields(&mut self, settings: &Settings) {
    let elapsed_seconds = self.initial.elapsed().map(|dur| dur.as_secs_f32()).unwrap_or(0.0);
    let total_season = settings.day_duration * settings.days_in_season as f32;
    let total_year = total_season * 4.0;

    self.time_of_year = (elapsed_seconds % total_year) / total_year;
    self.time_of_season = (elapsed_seconds % total_season) / total_season;
    self.time_of_day = (elapsed_seconds % settings.day_duration) / settings.day_duration;
    self.long.elapsed_seconds = self.long.last_timestamp.elapsed().map(|dur| dur.as_secs_f32()).unwrap_or(0.0);
    self.long.last_timestamp = SystemTime::now();
    /*
    println!("Times: \nelapsed_seconds: {}\n full_year: {}\n full_season: {}\n day: settings.day_duration: {}\n time_of_year: {}\n time_of seasion: {}\n time_of_day:{}",
      elapsed_seconds,
      total_year,
      total_season,
      settings.day_duration,
      self.time_of_year,
      self.time_of_season,
      self.time_of_day
   );
   */

  }
}

impl Default for Time {
  fn default() -> Self {
    Time {
      elapsed_seconds: 0.0,
      last_timestamp: std::time::SystemTime::now()
    }
  }
}

impl Default for Timers {
  fn default() -> Self {
    Timers {
      precize: Time::default(),
      long: Time::default(),
      initial: std::time::SystemTime::now(),
      day_of_year: 0,
      day_of_season: 0,
      current_season: Season::Spring,
      time_of_year: 0.0, 
      time_of_season: 0.0, 
      time_of_day: 0.0, 
    }
  }
}

impl From<u16> for Season {
  fn from(s: u16) -> Self {
    match s {
      0 => Season::Spring,
      1 => Season::Summer,
      2 => Season::Autumn,
      3 => Season::Winter,
      _ => panic!("incorrect seasion id"),
    }
  }
}


pub fn update_precise_timer(resources: &mut Resources) {
  if let Some(mut timers) = resources.get_mut::<Timers>() {
    timers.precize.elapsed_seconds = timers.precize.last_timestamp.elapsed().map(|dur| dur.as_secs_f32()).unwrap_or(0.0);
    timers.precize.last_timestamp = SystemTime::now();
  } else {
    println!("Error on update timers resource");
  }
}

pub fn update_long_times(resources: &mut Resources) {
  if let Some(settings) = resources.get::<Settings> () {
    if let Some(mut timers) = resources.get_mut::<Timers>() {
      timers.update_fields(&settings);
    } else {
      println!("Error on update timers resource");
    }
  }
}

#[system]
pub fn timer_update(#[resource] timers: &mut Timers, #[resource] settings: &Settings) {
  timers.update_fields(settings)
}
