use legion::*;
use legion::systems::CommandBuffer;
use super::timer::Timers;
use super::settings::Settings;
use serde::{ Serialize, Deserialize };
use super::interpolations::lerp_map;
use super::climate::Weather;
use super::movements::{ Vector3, Position };
use super::map::Map;
use std::ops::{Sub, Add, Div};
use rand::distributions::{ uniform::SampleUniform, Distribution, Uniform};
use num::traits::Zero;

static TREES_CONFIG: &str = include_str!("../config/trees.yaml");

#[derive(Clone, Serialize, Deserialize)]
struct GrowEffectiviness {
  starting_temp: f32,
  ending_temp: f32,
  distribution: Vec<f32>
}

pub struct Offspring {
  pub amount: u16,
  pub maturity: f32,
}

fn max<T: PartialOrd>(v: T, max: T) -> T {
  if v > max {
    v
  } else {
    max
  }
}

fn min<T: PartialOrd>(v: T, min: T) -> T {
  if v < min {
    v
  } else {
    min
  }
}

fn clamp<T: PartialOrd>(v: T, minimun: T, maximum: T) -> T {
  max(min(v, maximum), minimun)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TreeType {
  name: String,
  fruit_name: Option<String>,
  maturity_years: f32,
  blossom_start_year_time: f32,
  ripening_start_year_time: f32,
  fall_start_year_time: f32,
  sleep_start_year_time: f32,
  fertility: f32, // how much trees will grow next year
  fruit_amount: f32, // how much average grown tree gives
  growth_speed: f32,
  max_size: f32
}

impl TreeType {
  fn offspring(&self, props: &TreeProperties) -> Offspring {
    let size_mul = props.size / self.max_size;
    let amount = self.fruit_amount * size_mul;
    Offspring {
      amount: amount as u16,
      maturity: 0.0
    }
  }
}

struct Fruit {
  name: String
}

fn map_to_effectiveness(temp: f32, effectiveness: &GrowEffectiviness) -> f32 {
  let t = clamp(
    (temp - effectiveness.starting_temp) / (effectiveness.ending_temp - effectiveness.starting_temp),
    0.0, 1.0
  );
  lerp_map(t, &effectiveness.distribution)
}


#[derive(Serialize, Deserialize)]
pub struct TreeConfig {
  spieces: Vec<TreeType>
}

impl TreeConfig {
  fn load() -> Self {
    serde_yaml::from_str(TREES_CONFIG).unwrap()
  }
}

#[derive(PartialEq, Clone, Debug)]
enum TreeState{
  Sleep,
  Blossom,
  Ripening,
  Falling
}

pub struct TreeProperties {
  size: f32,
  age: f32,
  resources: f32,
  state: TreeState,
  negative_temprature_constant_time: f32
}

fn select_state(t: f32, tree: &TreeType) -> TreeState {
  use TreeState::*;
  let thresholds = [
    tree.sleep_start_year_time - 1.0,
    tree.blossom_start_year_time, 
    tree.ripening_start_year_time,
    tree.fall_start_year_time,
    tree.sleep_start_year_time,
    tree.blossom_start_year_time + 1.0,
  ];

  for i in 0..thresholds.len() - 1 {
    let min = thresholds[i];
    let max = thresholds[i+1];
    if t >= min && t < max {
      return match i {
        0 => Sleep,
        1 => Blossom,
        2 => Ripening,
        3 => Falling,
        4 => Sleep,
        _ => { panic!("wrong choice"); }
      };
    } else {
    }
  }

  panic!("time out of range {}", t);
}

fn state_transition(year_time: f32, tree: &TreeType) -> TreeState {
  select_state(year_time, tree)
}

fn inmature_state_transition(year_time: f32, tree: &TreeType) -> TreeState {
  use TreeState::*;
  match select_state(year_time, tree) {
    Blossom => {
      Ripening
    },
    state => state
  }
}

/* OLD VERSION
fn state_transition(resources: f32, state: TreeState, tree: &TreeType) -> (f32, TreeState) {
  use TreeState::*;
  match state {
    Sleep  if tree.wakeup_cost < resources  => {
       (resources - tree.wakeup_cost, Awaked)
    },
    Awaked if tree.blossom_cost < resources  => {
       (resources - tree.blossom_cost, Blossom)
    },
    Blossom if tree.ovary_cost < resources => {
       (resources - tree.ovary_cost, Ripening)
    },
    Ripening if tree.ripening_cost < resources => {
       (resources - tree.ripening_cost, Falling)
    },
    Falling if tree.sleep_cost < resources => {
       (resources - tree.sleep_cost, Sleep)
    },
    _ => (resources, state)
  }
}
*/

pub fn place_tree_zero(world: &mut World) {
  let position = Position::zero();
  let tc = TreeConfig::load();
  let my_tree = tc.spieces[0].clone();
  world.push((
      position,
      my_tree,
      TreeProperties {
        age: 0.0,
        resources: 20.0,
        size: 0.0,
        state: TreeState::Sleep,
        negative_temprature_constant_time: 0.0
      }
  ));
}
pub fn place_tree_test(
  world: &mut World, 
  p: Position, 
  age: f32, 
  size: f32
) {
  let position = Position::zero();
  let tc = TreeConfig::load();
  let my_tree = tc.spieces[0].clone();
  world.push((
      position,
      my_tree,
      TreeProperties {
        age,
        resources: 20.0,
        size,
        state: TreeState::Sleep,
        negative_temprature_constant_time: 0.0
      }

  ));
}
pub fn place_tree(pusher: &mut CommandBuffer, position: Position, my_tree: TreeType) {
  let tc = TreeConfig::load();
  pusher.push((
      position,
      my_tree,
      TreeProperties {
        age: 0.0,
        resources: 20.0,
        size: 0.0,
        state: TreeState::Sleep,
        negative_temprature_constant_time: 0.0
      }

  ));
}

fn get_uniform_around<T>(point: T, thres: T) -> T 
where
  T:SampleUniform + Copy + Add<Output =T>+Sub<Output =T>+From<f32>
{

  let dist = Uniform::from(point - thres .. point + thres);
  let mut rng = rand::thread_rng();
  dist.sample(&mut rng)
}

fn get_position_around_tree(position: &Position, tree_type: &TreeType, tree: &TreeProperties) -> Position {
  let radius = tree.size * 0.7;
  let random_shift = Position(Vector3::new(
      get_uniform_around(0.0, radius), 
      get_uniform_around(0.0, radius), 
      0.0));
  *position + random_shift

}

fn seed_new_trees(position: &Position, tree: &TreeProperties, tree_type: &TreeType, pusher: &mut CommandBuffer) {
  let amount = get_uniform_around(tree_type.fertility, tree_type.fertility / 3.0) as u16;
  println!("seed {} trees", amount);
  for _ in 0..amount {
    let new_position = get_position_around_tree(position, tree_type, tree);
    place_tree(pusher, new_position, tree_type.clone());
  }
}

fn place_offsprings(entity: &Entity, tree: &TreeProperties, tree_type: &TreeType, command_buffer: &mut CommandBuffer) {
  println!("place Offsprings");

  command_buffer.add_component(*entity, tree_type.offspring(tree));
}
fn remove_offsprings(entity: &Entity, command_buffer: &mut CommandBuffer) {
  println!("remove Offsprings");
  command_buffer.remove_component::<Offspring>(*entity)
}

#[system(for_each)]
pub fn drop_fruits(
  position: &Position,
  tree_type: & TreeType,
  tree: &TreeProperties,
  offspring: &mut Offspring,
  command_buffer: &mut CommandBuffer,
) {
  if let Some(fruit_name) = &tree_type.fruit_name {
    if offspring.maturity > 0.7 && offspring.amount > 0 {
      let lvl = 0.7;
      let t = clamp((offspring.maturity - lvl) / (1.0 - lvl), 0.0, 1.0) / 10.;
      let t = t * t;
      let amount = (offspring.amount as f32 * t) as u16;
      let amount = min(amount, offspring.amount);
      println!("mature oak spawns {}/{} {} ", amount, offspring.amount, fruit_name);
      offspring.amount -= amount;
      for _ in 0..amount {
        command_buffer.push((Fruit{name: fruit_name.to_owned()}, get_position_around_tree(position, tree_type, tree)));
      }
    }
  }
}

#[system(par_for_each)]
pub fn update_offspring(
  tree_type: & TreeType,
  offspring: &mut Offspring,
  #[resource] time: &Timers,
){
  let cur_time = time.time_of_year - tree_type.ripening_start_year_time;
  let cur_time_normalized = cur_time / (tree_type.fall_start_year_time - tree_type.ripening_start_year_time);
  println!("cur_time_normalized: {}", cur_time_normalized);
  offspring.maturity = cur_time_normalized;
}

#[system(for_each)]
pub fn update_trees(
  entity: &Entity,
  tree_type: &TreeType, 
  properties: &mut TreeProperties,
  position: &Position,
  command_buffer: &mut CommandBuffer,
  #[resource] time: &Timers,
  #[resource] settings: &Settings,
  ) {
  properties.age += time.long.elapsed_seconds;
  let years = properties.age / settings.seconds_in_year();

  let new_state = if years > tree_type.maturity_years {
    state_transition(time.time_of_year, tree_type)
  } else {
    inmature_state_transition(time.time_of_year, tree_type)
  };
  println!("{:?}, age-years: {}, old state: {:?}, new_state: {:?} {}",entity, years, properties.state, new_state, time.time_of_year);


  if properties.state == TreeState::Blossom && new_state == TreeState::Ripening {
    place_offsprings(entity, properties, tree_type, command_buffer);
  }

  if properties.state == TreeState::Falling && new_state == TreeState::Sleep {
    remove_offsprings(entity, command_buffer);
    if years > tree_type.maturity_years {
      seed_new_trees(position, properties, tree_type, command_buffer);
    }
  }

  properties.state = new_state;
  if matches!(properties.state, TreeState::Blossom | TreeState::Ripening) && properties.size <= tree_type.max_size {
    properties.size += tree_type.growth_speed * time.long.elapsed_seconds;
  }
}


/*
 *
DONT DELETE - POSSILBLY GOOD APPROACH FOR GRASS!!!
#[system(for_each)]
pub fn update_trees_complex(
  tree_type: &TreeType, 
  properties: &mut TreeProperties,
  position: &Position,
  #[resource] time: &Timers,
  #[resource] weather: &Weather,
  #[resource] map: &Map,
  ) {


  let tempreture = weather.current_tempreture;
  println!("ok T = {}", tempreture);
  if tempreture < 0.0 {
    properties.negative_temprature_constant_time += time.long.elapsed_seconds;
  } else {
    properties.negative_temprature_constant_time = 0.0;
  }

  if properties.negative_temprature_constant_time > tree_type.negative_tempreture_survival {
    println!("this tree died");
  } else {

    let effectiveness = map_to_effectiveness(weather.current_tempreture, &tree_type.effectiveness);
    let soil_fertility = map.get_soil_fertility(position);
    let total_gain = effectiveness * soil_fertility * time.long.elapsed_seconds;
    println!("total gain: G={}, E={}, {}s, F = {}, state: {:?}", total_gain, effectiveness, time.long.elapsed_seconds, soil_fertility, properties.state);
    let total_gain = if properties.state == TreeState::Awaked {
      let groth = total_gain * tree_type.groth_blossom_ratio;
      properties.size += groth;
      total_gain - groth
    } else { total_gain };

    properties.resources += total_gain;
    properties.age += time.long.elapsed_seconds;
    let (new_resources, new_state) = state_transition(
      properties.resources, 
      properties.state.clone(),
      tree_type
    );
    properties.state = new_state;
    properties.resources = new_resources;
  }
}
*/

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn select_state_test() {
    let trees = TreeConfig::load();
    assert_eq!(select_state(0.0, &trees.spieces[0]), TreeState::Sleep);
    assert_eq!(select_state(0.13, &trees.spieces[0]), TreeState::Blossom);
    assert_eq!(select_state(0.21, &trees.spieces[0]), TreeState::Ripening);
    assert_eq!(select_state(0.501, &trees.spieces[0]), TreeState::Falling);
    assert_eq!(select_state(0.701, &trees.spieces[0]), TreeState::Sleep);
  }
  #[test]
  fn check_clamp() {
    assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
    assert_eq!(clamp(0.0, 0.0, 1.0), 0.0);
    assert_eq!(clamp(1.0, 0.0, 1.0), 1.0);
  }

  #[test]
  fn check_min() {
    assert_eq!(min(0.5,1.0), 0.5);
    assert_eq!(min(1.4,1.0), 1.0);
    assert_eq!(min(1.0,1.0), 1.0);
  }
  #[test]
  fn check_max() {
    assert_eq!(max(0.5,1.0), 1.0);
    assert_eq!(max(1.4,1.0), 1.4);
    assert_eq!(max(1.0,1.0), 1.0);
  }
}
