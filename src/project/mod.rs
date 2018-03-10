mod model;
mod resource;
mod project;
mod parent_project;

pub use self::project::YYP;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ID(String);
