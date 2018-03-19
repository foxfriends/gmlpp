use std::fs::File;
use std::path::{Path, PathBuf};

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
enum YY {
    #[serde(rename="GMObject")]
    Object(Object),
    #[serde(rename="GMScript")]
    Script(Script),
}

#[derive(Clone, Debug)]
pub struct Resource {
    path: PathBuf,
    resource: YY,
}

impl Resource {
    /// Loads a resource from the `.yy` file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Resource, Error> {
        let file = File::open(path.as_ref()).map_err(|_| Error::missing_resource(path.as_ref()))?;
        let resource = serde_json::from_reader(file)?;
        Ok(
            Self {
                path: path.as_ref().parent().unwrap().to_owned(),
                resource,
            }
        )
    }

    /// Lists all this resource's associated `.gml` and `.gmlpp` files
    pub fn sources(&self) -> Vec<Source> {
        use self::YY::*;
        match &self.resource {
            &Object(ref object) => object.sources().into_iter().map(|source| source.resolved_to(self.path.clone())).collect(),
            &Script(ref script) => script.sources().into_iter().map(|source| source.resolved_to(self.path.clone())).collect(),
            _ => vec![],
        }
    }
}
