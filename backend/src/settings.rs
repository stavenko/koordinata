pub struct Settings {
  pub days_in_season: u16,
  pub day_duration: f32,
  pub climate_zone: String,
}

impl Default for Settings {
  fn default() -> Self {
    let seconds_per_hour = 60_f32 * 60.0;
    Settings {
      days_in_season: 10, // 10 days per season
      day_duration: 12.0 * seconds_per_hour, // 12 hours pre day
      climate_zone: "moderate".into()
    }
  }
}

impl Settings {
  pub fn seconds_in_year(&self) -> f32 {
    (self.days_in_season as f32) * 4.0 * self.day_duration
  }
}
