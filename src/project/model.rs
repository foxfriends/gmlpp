#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    #[serde(rename="GMProject")]
    Project,
    #[serde(rename="GMProjectParent")]
    ParentProject,
}
