use geo::{ Coordinate, Polygon, Rect };
use super::movements::Position;
pub enum GroundType {
  Sand,
  Soil,
  Rocks,
  Dirt,
  Water,
  Asphalt,
  Concreete,
}


struct GeographicFeature {
  tp: GroundType,
  area: Polygon<f32>
}


pub struct Map(Vec<GeographicFeature>);

impl Map {
  pub fn test_square(width: f32, height: f32) -> Self {
    let polygon = Polygon::from(
      Rect::new(
        Coordinate{ x: -width / 2.0, y: -height /2.0}, 
        Coordinate{ x: width / 2.0, y: height /2.0}, 
    ));
    let soil = GeographicFeature {
      tp: GroundType::Soil,
      area: polygon
    };

    Map(vec!(soil))
  }

  pub fn get_soil_fertility(&self, _position: &Position) -> f32 {
    1.0
  }
}
