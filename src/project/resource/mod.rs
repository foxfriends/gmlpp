use std::fs::File;
use std::path::Path;

use serde_json;

use error::Error;
use super::source::Source;

mod tag;
mod object;
mod script;
mod event;

pub use self::tag::*;
pub use self::object::*;
pub use self::script::*;
pub use self::event::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag="modelName")]
pub enum Resource {
    #[serde(rename="GMObject")]
    Object(Object),
    #[serde(rename="GMScript")]
    Script(Script),
}

impl Resource {
    /// Loads a resource from the `.yy` file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Resource, Error> {
        let file = File::open(path.as_ref()).map_err(|_| Error::missing_resource(path))?;
        serde_json::from_reader(file).map_err(From::from)
    }

    /// Lists all this resource's associated `.gml` and `.gmlpp` files
    pub fn sources(&self) -> Vec<Source> {
        vec![]
    }
}
