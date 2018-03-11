use std::path::PathBuf;

/// A `.gml` source file and it's corresponding `.gmlpp` and helper files if required
#[derive(Clone, Debug)]
pub struct Source {
    gml: String,
    gmlpp: Option<String>,
    helper: Option<String>,
}

impl Source {
    /// The path to the `.gml` file for this source
    pub fn gml(&self) -> &String {
        &self.gml
    }

    /// The path to the `.gmlpp` file for this source
    pub fn gmlpp(&self) -> &Option<String> {
        &self.gmlpp
    }

    /// The path to the helper `.gml` file for this source
    pub fn helper(&self) -> &Option<String> {
        &self.helper
    }

    /// Determines the source files if the path is a `.gmlpp` file
    pub fn from(path: PathBuf) -> Option<Self> {
        if path.extension()?.to_str()? == ".gmlpp" {
            Some(
                Self {
                    gml: path.with_extension("gml").as_os_str().to_str()?.to_owned(),
                    gmlpp: Some(path.as_os_str().to_str()?.to_owned()),
                    helper: None,
                }
            )
        } else {
            None
        }
    }
}


