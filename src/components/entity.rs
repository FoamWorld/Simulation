use std::any::Any;
use std::collections::HashMap;

pub trait Entity {
    fn get_property(&self, key: String);
    fn set_property(&mut self, key: String, value: Any);
}

struct Humainiod {
    name: String
}
