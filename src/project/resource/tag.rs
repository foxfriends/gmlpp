use std::collections::HashMap;

use super::super::ID;
use super::Resource;
use error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceTag {
    #[serde(rename="Key")]
    key: ID,
    #[serde(rename="Value")]
    value: ResourceValue,
}

impl ResourceTag {
    /// The type of resource that is being tagged
    pub fn resource_type(&self) -> ResourceType {
        self.value.resource_type
    }

    /// Retrieves the resource from the cache or file system
    pub fn resource(&self, base_path: &str, cache: &mut HashMap<ID, Resource>) -> Result<Resource, Error> {
        match cache.get(&self.key).cloned() {
            Some(resource) => Ok(resource),
            None => {
                let resource = Resource::new(format!("{}/{}", base_path, self.value.resource_path.replace("\\", "/")))?;
                cache.insert(self.key.clone(), resource.clone());
                Ok(resource)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
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
