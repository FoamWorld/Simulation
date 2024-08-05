use std::any::Any;

/// Trait for simulating real objects.
/// Tools for management are not included.
pub trait Referent {
    fn get(&self, key: String) -> Option<&dyn Any>;
}

/// Empty. Prepared to be overriden.
pub struct Void {}
impl Referent for Void {
    fn get(&self, _: String) -> Option<&dyn Any> {
        return None;
    }
}

pub struct Barrier {
    pub level: u8,
}
impl Referent for Barrier {
    fn get(&self, key: String) -> Option<&dyn Any> {
        return if key.as_str() == "level" {
            Some(&self.level)
        } else {
            None
        };
    }
}

pub struct Container {
    refer: Box<dyn Referent>,
}
impl Referent for Container {
    fn get(&self, key: String) -> Option<&dyn Any> {
        return if key.as_str() == "refer" {
            Some(&self.refer)
        } else {
            return None;
        };
    }
}

/// A last choice. You can try serializing.
pub struct Decoration {
    pub essence: String,
}
impl Referent for Decoration {
    fn get(&self, key: String) -> Option<&dyn std::any::Any> {
        return if key.to_string() == "essence" {
            Some(&self.essence)
        } else {
            None
        };
    }
}
