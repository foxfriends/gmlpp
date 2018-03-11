use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;

use serde_json;

use error::Error;
use super::parent_project::ParentProject;
use super::resource::{Resource, ResourceTag, ResourceType};
use super::model::Model;
use super::source::Source;
use super::ID;

/// Data representation of a `.yyp` file
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Project {
    id: ID,
    #[serde(rename="modelName")]
    model_name: Model,
    mvc: String,
    #[serde(rename="IsDnDProject")]
    is_dnd_project: bool,
    configs: Vec<()>,
    option_ecma: bool,
    #[serde(rename="parentProject")]
    parent_project: ParentProject,
    resources: Vec<ResourceTag>,
    script_order: Vec<String>,
    tutorial: String,
}

/// A GameMaker project
#[derive(Clone, Debug)]
pub struct YYP {
    project_file: String,
    project: Project,
    resources: RefCell<HashMap<ID, Resource>>,
}

impl YYP {
    /// Loads a GameMaker Studio 2 project from a `.yyp` file
    pub fn new(project_file: String) -> Result<Self, Error> {
        let f = File::open(project_file.clone()).map_err(|_| Error::NoProject)?;
        serde_json::from_reader(f)
            .map(move |project| Self { project_file, project, resources: RefCell::new(HashMap::new()) })
            .map_err(From::from)
    }

    /// The directory this project file is in
    pub fn directory(&self) -> &Path {
        Path::new(&self.project_file).parent().unwrap()
    }

    /// Finds all the source files for all resources
    pub fn sources(&self) -> Vec<Source> {
        self.project
            .resources
            .iter()
            .filter(|tag| 
                tag.resource_type() == ResourceType::Object ||
                tag.resource_type() == ResourceType::Script
            )
            .map(|tag| tag.resource(self.directory().to_str().unwrap(), &mut self.resources.borrow_mut()).unwrap())
            .flat_map(|resource| resource.sources())
            .collect()
    }
}

