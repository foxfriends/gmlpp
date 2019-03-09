use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::sync::mpsc::channel;

use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

use gmlpp::AST;
use project::{Project, Source};
use error::Error;

/// Performs compilation of `.gmlpp` files within a project
#[derive(Clone, Debug)]
pub struct Compiler {
    project: Project,
}

impl Compiler {
    /// Creates a new instance of the Compiler, linked to a project
    pub fn new(project: Project) -> Self {
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
            DebouncedEvent::Write(path) => self.compile(Source::from(path)),
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
        if source.gmlpp().exists() {
            println!("Compiling source: {:?}", source);
            let file = File::open(source.gmlpp())?;
            match AST::from_reader(file) {
                Ok(ast) => {
                    let mut outfile = File::create(source.gml())?;
                    write!(outfile, "{}", ast.print())?;
                }
                Err(error) => {
                    eprintln!("Error parsing file '{}': {:?}", source.gmlpp().to_string_lossy(), error);
                }
            }
            // do compile
        }
        Ok(())
    }
}
