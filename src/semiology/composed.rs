use bevy_ecs::component::Component;
use serde_json::Value;

#[derive(Component, Clone)]
pub struct ComposedConcept {
    pub value: Value,
}
impl ComposedConcept {
    pub fn new() -> Self {
        ComposedConcept { value: Value::Null }
    }
}
