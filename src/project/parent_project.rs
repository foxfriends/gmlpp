use super::resource::ResourceTag;
use super::model::Model;
use super::ID;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParentProject {
    id: ID,
    #[serde(rename="modelName")]
    model_name: Model,
    mvc: String,
    #[serde(rename="alteredResources")]
    altered_resources: Vec<ResourceTag>,
    #[serde(rename="hiddenResources")]
    hidden_resources: Vec<ResourceTag>,
    #[serde(rename="projectPath")]
    project_path: String,
}
