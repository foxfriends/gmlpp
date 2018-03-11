use std::path::PathBuf;
use std::fs::File;
use std::ffi::OsStr;
use std::time::Duration;
use std::sync::mpsc::channel;

use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

use project::YYP;
use error::Error;

/// Performs compilation of `.gmlpp` files within a project
#[derive(Clone, Debug)]
pub struct Compiler {
    project: YYP,
}

impl Compiler {
    /// Creates a new instance of the Compiler, linked to a project
    pub fn new(project: YYP) -> Self {
        Self{ project }
    }

    /// Watches the project files, compiling the gmlpp files to gml
    pub fn watch(self) -> Result<(), Error> {
        self.compile_all()?;
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
        watcher.watch(self.project.directory(), RecursiveMode::Recursive)?;
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
            DebouncedEvent::Write(path) => self.compile(path),
            _ => Ok(()),
        }
    }

    /// Compiles all the watched `.gmlpp` files
    fn compile_all(&self) -> Result<(), Error> {
        Ok(())
    }

    /// Compiles a `.gmlpp` file to it's corresponding `.gml` file
    fn compile(&self, path: PathBuf) -> Result<(), Error> {
        match path.extension().and_then(OsStr::to_str) {
            Some(".gmlpp") => {
                let gml_path = path.with_extension("gml");
                let file = File::open(path)?;
                let outfile = File::create(gml_path)?;
                // do compile
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
