use legion::*;
use nalgebra as na;
use std::ops::{ Add, AddAssign, Mul };
use num::traits::Zero;
use super::timer::Timers;

pub type Vector3 = na::Vector3<f32>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(pub Vector3);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity(pub Vector3);
// struct Distance(Vector3);
// struct Path(Vec<Vector3>);



impl AddAssign for Position {
  fn add_assign(&mut self, p: Position) {
    self.0 += p.0;
  }
}

impl Mul<f32> for Velocity {
  type Output = Position;

  fn mul(self, time: f32) -> Self::Output {
    Position(self.0 * time)
  }
}

impl Add<Position> for Position {
  type Output = Position;

  fn add(self, p: Position) -> Self::Output {
    Position(self.0 + p.0)
  }
}

impl Zero for Position {
  fn zero()->Self {
    Position(Vector3::new(0.0, 0.0, 0.0))
  }

  fn is_zero(&self) -> bool {
    self.0 == Vector3::new(0.0, 0.0, 0.0)
  }
  fn set_zero(&mut self) {
    self.0 = Vector3::new(0.0, 0.0, 0.0);
  }

}
#[system(for_each)]
pub fn update_positions(pos: &mut Position, vel: &Velocity, #[resource] timers: &Timers) {
  *pos += (*vel) * timers.precize.elapsed_seconds;
}


