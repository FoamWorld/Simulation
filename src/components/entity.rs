use super::character::Character;
use super::memory::Memory;
use std::any::Any;
use std::collections::BTreeMap;

pub trait Entity {
    fn get_property(&self, key: String) -> Option<&Box<dyn Any>>;
    fn set_property(&mut self, key: String, value: Box<dyn Any>);
    fn damage_calc(&self, dmg: f32, dmg_type: String) -> f32;
}

pub struct Humanoid {
    pub properties: BTreeMap<String, Box<dyn Any>>,
    pub dmg_ratio: BTreeMap<String, f32>,
    pub character: Character,
    pub memory: Memory,
    pub relations: BTreeMap<String, String>,
    // body_slots
}
impl Humanoid {
    pub fn default() -> Self {
        Humanoid {
            properties: BTreeMap::<String, Box<dyn Any>>::new(),
            dmg_ratio: BTreeMap::<String, f32>::new(),
            character: Character::new(),
            memory: Memory::new(),
            relations: BTreeMap::<String, String>::new(),
        }
    }
}
impl Entity for Humanoid {
    fn get_property(&self, key: String) -> Option<&Box<dyn Any>> {
        return self.properties.get(&key);
    }
    fn set_property(&mut self, key: String, value: Box<dyn Any>) {
        self.properties.insert(key, value);
    }
    fn damage_calc(&self, dmg: f32, dmg_type: String) -> f32 {
        return match self.dmg_ratio.get(&dmg_type) {
            Some(x) => x * dmg,
            None => dmg,
        };
    }
}
