use std::fs::File;
use std::time::Duration;
use std::sync::mpsc::channel;

use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

use gmlpp::AST;
use project::{YYP, Source};
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
            DebouncedEvent::Write(path) => {
                for source in Source::from(path) {
                    self.compile(source)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Compiles all the `.gmlpp` files in the project
    pub fn compile_all(&self) -> Result<(), Error> {
        for source in self.project.sources() {
            self.compile(source)?;
        }
        Ok(())
    }

    /// Compiles a `.gmlpp` file to it's corresponding `.gml` file
    fn compile(&self, source: Source) -> Result<(), Error> {
        for gmlpp in source.gmlpp() {
            println!("{}{}", self.project.directory().to_str().unwrap(), gmlpp);
            let file = File::open(format!("{}{}", self.project.directory().to_str().unwrap(), gmlpp))?;
            let ast = AST::from_reader(file);
            let outfile = File::create(source.gml())?;
            // do compile
        }
        Ok(())
    }
}
