use super::ID;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceTag {
    #[serde(rename="Key")]
    key: ID,
    #[serde(rename="Value")]
    value: ResourceValue,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ResourceType {
    #[serde(rename="GMObject")]
    Object,
    #[serde(rename="GMFolder")]
    Folder,
    #[serde(rename="GMScript")]
    Script,
    #[serde(rename="GMSprite")]
    Sprite,
    #[serde(rename="GMRoom")]
    Room,
    #[serde(rename="GMTileSet")]
    TileSet,
    #[serde(rename="GMFont")]
    Font,
    #[serde(rename="GMSound")]
    Sound,
    #[serde(rename="GMShader")]
    Shader,
    #[serde(rename="GMTimeline")]
    Timeline,
    #[serde(rename="GMNote")]
    Note,
    #[serde(rename="GMIncludedFile")]
    IncludedFile,
    #[serde(rename="GMExtension")]
    Extension,
    #[serde(rename="GMMainOptions")]
    MainOptions,
    #[serde(rename="GMLinuxOptions")]
    LinuxOptions,
    #[serde(rename="GMMacOptions")]
    MacOptions,
    #[serde(rename="GMWindowsOptions")]
    WindowsOptions,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceValue {
    id: ID,
    #[serde(rename="configDeltas")]
    config_deltas: Option<Vec<String>>,
    #[serde(rename="resourcePath")]
    resource_path: String,
    #[serde(rename="resourceType")]
    resource_type: ResourceType,
}
