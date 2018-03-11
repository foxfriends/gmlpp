use std::fs::File;
use std::path::Path;

use serde_json;

use error::Error;
use super::parent_project::ParentProject;
use super::resource::ResourceTag;
use super::model::Model;
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
}

impl YYP {
    /// Loads a GameMaker Studio 2 project from a `.yyp` file
    pub fn new(project_file: String) -> Result<Self, Error> {
        let f = File::open(project_file.clone())?;
        serde_json::from_reader(f)
            .map(move |project| Self { project_file, project })
            .map_err(From::from)
    }

    /// The directory this project file is in
    pub fn directory(&self) -> &Path {
        Path::new(&self.project_file).parent().unwrap()
    }
}

