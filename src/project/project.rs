use std::path::Path;
use std::fs::File;
use std::time::Duration;
use std::sync::mpsc::channel;

use serde_json;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

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

    /// Watches the project files, compiling the gmlpp files to gml
    pub fn watch(self) -> Result<(), Error> {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
        watcher.watch(Path::new(&self.project_file).parent().unwrap(), RecursiveMode::Recursive)?;
        loop {
            match rx.recv() {
                Ok(event) => self.handle(event)?,
                Err(_) => break,
            }
        }
        Ok(())
    }

    /// Handles a notification received from the watcher
    fn handle(&self, event: DebouncedEvent) -> Result<(), Error> {
        match event {
            _ => Ok(())
        }
    }
}

