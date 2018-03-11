use super::super::ID;
use super::super::model::Model;

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
