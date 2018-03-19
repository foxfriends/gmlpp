#![feature(io)]

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate notify;

mod project;
mod gml;
mod gmlpp;
mod compiler;
mod error;

use std::env;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), self::error::Error> {
    let args = env::args();
    // TODO: might be nice to predict project file names if they are not supplied
    let project_file = args.into_iter().nth(1).ok_or(self::error::Error::ArgumentError)?;
    let project = self::project::Project::new(project_file)?;
    let compiler = self::compiler::Compiler::new(project);
    compiler.compile_all()?;
    compiler.watch()
}
