#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    #[serde(rename="GMProject")]
    Project,
    #[serde(rename="GMProjectParent")]
    ParentProject,
    #[serde(rename="GMEvent")]
    Event,
    #[serde(rename="GMScript")]
    Script,
    #[serde(rename="GMObject")]
    Object,
    #[serde(rename="GMPath")]
    Path,

    // sprites
    #[serde(rename="GMSprite")]
    Sprite,
    #[serde(rename="GMSpriteFrame")]
    SpriteFrame,
    #[serde(rename="GMImageLayer")]
    ImageLayer,

    // rooms
    #[serde(rename="GMRoom")]
    Room,
    #[serde(rename="GMRoomSettings")]
    RoomSettings,
    #[serde(rename="GMRoomViewSettings")]
    RoomViewSettings,
    #[serde(rename="GMRoomPhysicsSettings")]
    RoomPhysicsSettings,
    #[serde(rename="GMRInstanceLayer")]
    RInstanceLayer,
    #[serde(rename="GMRInstance")]
    RInstance,
    #[serde(rename="GMRBackgroundLayer")]
    RBackgroundLayer,
    #[serde(rename="GMRView")]
    RView,

    #[serde(rename="GMFolder")]
    Folder,
}
