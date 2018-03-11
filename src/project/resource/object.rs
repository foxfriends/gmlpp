use super::super::ID;
use super::super::model::Model;
use super::Event;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all="camelCase")]
pub struct Object {
    id: ID,
    mvc: String,
    name: String,
    event_list: Vec<Event>,
    mask_sprite_id: ID,
    parent_object_id: ID,
    persistent: bool,
    physics_angular_damping: f32,
    physics_density: f32,
    physics_friction: f32,
    physics_group: i32,
    physics_kinematic: bool,
    physics_linear_damping: f32,
    physics_object: bool,
    physics_restitution: f32,
    physics_sensor: bool,
    physics_shape: i32,
    physics_shape_points: Option<()>,
    physics_start_awake: bool,
    solid: bool,
    sprite_id: ID,
    visible: bool,
}
