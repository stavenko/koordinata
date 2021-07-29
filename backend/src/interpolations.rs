pub fn circle_map(t: f32, values: &[f32]) -> f32 {
  let step = 1.0 / values.len() as f32 ;
  let floored = (t / step).floor() as usize ;
  if floored == values.len() {
    values[0]
  } else {
    let distance_between = t % step;
    let param_between = distance_between / step;
    let ceiled = (floored + 1) % values.len();
    let a = values[floored];
    let b = values[ceiled];
    (1.0 - param_between) * a + param_between * b
  }
}

pub fn lerp_map(t: f32, values: &[f32]) -> f32 {
  let step = 1.0 / (values.len() - 1) as f32 ;
  let floored = (t / step).floor() as usize ;
  let distance_between = t % step;
  let param_between = distance_between / step;
  if floored == values.len() -1 {
    values[floored]
  } else {
    let ceiled = floored + 1;
    let a = values[floored];
    let b = values[ceiled];
    (1.0 - param_between) * a + param_between * b
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn check_that_lerp_with_two_points_is_ok() {
    let values = vec!(1.0, 2.0);
    let v = lerp_map(0.0, &values);

    assert_eq!(v, 1.0);

    let v = lerp_map(1.0, &values);

    assert_eq!(v, 2.0);

    let v = lerp_map(0.5, &values);

    assert_eq!(v, 1.5);

    let v = lerp_map(0.25, &values);

    assert_eq!(v, 1.25);

    let v = lerp_map(0.75, &values);

    assert_eq!(v, 1.75);
  }
  #[test]
  fn check_that_interpolation_with_two_points_is_ok() {
    let values = vec!(1.0, 2.0);
    let v = circle_map(0.0, &values);

    assert_eq!(v, 1.0);

    let v = circle_map(1.0, &values);

    assert_eq!(v, 1.0);

    let v = circle_map(0.5, &values);

    assert_eq!(v, 2.0);

    let v = circle_map(0.25, &values);

    assert_eq!(v, 1.5);

    let v = circle_map(0.75, &values);

    assert_eq!(v, 1.5);
  }

  #[test]
  fn check_that_interpolation_with_four_points_is_ok() {
    let values = vec!(1.0, 2.0, 5.0, 50.0);
    let v = circle_map(0.0, &values);

    assert_eq!(v, 1.0);

    let v = circle_map(1.0, &values);

    assert_eq!(v, 1.0);

    let v = circle_map(0.5, &values);

    assert_eq!(v, 5.0);

    let v = circle_map(0.25, &values);

    assert_eq!(v, 2.0);

    let v = circle_map(0.75, &values);

    assert_eq!(v, 50.0);
  }
}
