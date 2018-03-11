use super::super::ID;
use super::super::model::Model;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    id: ID,
    #[serde(rename="modelName")]
    model_name: Model,
    mvc: String,
    #[serde(rename="IsDnD")]
    is_dnd: bool,
    #[serde(rename="collisionObjectId")]
    collision_object_id: ID,
    enumb: i32,
    eventtype: i32,
    m_owner: ID,
}

impl Event {
    pub fn event_type(&self) -> EventType {
        use self::EventType::*;
        match self.eventtype {
            0 => Create(self.enumb),
            3 => Step(self.enumb),
            4 => Collision(self.id.clone()),
            7 => Other(self.enumb),
            8 => Draw(self.enumb),
            12 => CleanUp(self.enumb),
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug)]
enum EventType {
    Create(i32),
    Step(i32),
    Collision(ID),
    Other(i32),
    Draw(i32),
    CleanUp(i32),
}

impl EventType {
    pub fn file_name(&self) -> String {
        use self::EventType::*;
        match self {
            &Create(num) => format!("Create_{}.gml", num),
            &Step(num) => format!("Step_{}.gml", num),
            &Collision(ref id) => format!("Collision_{}.gml", id),
            &Other(num) => format!("Other_{}.gml", num),
            &Draw(num) => format!("Draw_{}.gml", num),
            &CleanUp(num) => format!("CleanUp_{}.gml", num),
        }
    }
}
