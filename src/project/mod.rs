use std::fmt;

mod model;
mod resource;
mod project;
mod parent_project;
mod source;

pub use self::project::Project;
pub use self::source::Source;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ID(String);

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
