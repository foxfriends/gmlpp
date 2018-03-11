use super::super::ID;
use super::super::source::Source;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Script {
    id: ID,
    mvc: String,
    name: String,
    #[serde(rename="IsCompatibility")]
    is_compatibility: bool,
    #[serde(rename="IsDnD")]
    is_dnd: bool,
}

impl Script {
    /// Returns the source files for this script. Should be just the one.
    pub fn sources(&self) -> Vec<Source> {
        vec![Source::from(format!("{}.gml", self.name))]
    }
}
