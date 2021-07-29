use legion::*;
use serde::{ Serialize, Deserialize };
use super::timer::Timers;
use super::interpolations::circle_map;

static CLIMATE: &str = include_str!("../config/climate.yaml");

#[derive(Serialize, Deserialize)]
struct Zone {
  name: String,
  monthly_temp: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
struct ClimateSettings {
  zones: Vec<Zone>,
  dayly_tempreture_floating: f32
}

pub struct Weather {
  dayly_curve: Vec<f32>,
  pub current_tempreture: f32,
  yearly_curve: Vec<f32>
}


impl ClimateSettings {
}


#[system]
pub fn weather(#[resource] weather_resource: &mut Weather, #[resource] timers: &Timers) {
  weather_resource.current_tempreture = weather_resource.calculate_normal_tempreture(timers);
}

impl Weather {
  pub fn prepare(zone_name: &str) -> Self {
    let climate: ClimateSettings = serde_yaml::from_str(CLIMATE).unwrap();
    let dayly_curve = {
      let mut curve: Vec<f32> = Vec::new();
      for i in 0..24 {
        let cos: f32 = (std::f32::consts::PI * 2.0 * (i as f32) / 24.0).cos();
        curve.push(cos * climate.dayly_tempreture_floating);
      }
      curve
    };

    if let Some(zone) = climate.zones.iter().find(|zone| zone.name == zone_name) {

      Weather {
        current_tempreture: 0.0,
        dayly_curve,
        yearly_curve: zone.monthly_temp.clone()
      }
    } else {
      panic! ("cannot construct weather for zone {}", zone_name);
    }
  }

  fn calculate_normal_tempreture(&self, timers: &Timers) -> f32 {
    let dayly_fluctuation = circle_map(timers.time_of_day, &self.dayly_curve);
    // time of year - is a param from first day of spring to last day of winter
    let yearly_fluctuation = circle_map(timers.time_of_year, &self.yearly_curve);


    dayly_fluctuation + yearly_fluctuation
  }

}

#[cfg(test)]
mod test {
  use super::*;
  use super::super::settings::Settings;

  #[test]
  fn check_that_temp_is_ok() {
    let zone_name: String = "tropical".into();
    let w = Weather::prepare(&zone_name);
    let settings = Settings {
      days_in_season: 1,
      day_duration: 1.0,
      climate_zone: zone_name.to_owned()
    };
    let mut timers = Timers::default();
    timers.update_fields(&settings);
    let t = w.calculate_normal_tempreture(&timers);
    assert_eq!(t, 24.0);

  }
}
