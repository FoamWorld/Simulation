use crate::semiology::referent::{FunctionalTypes, Referent};

pub struct Tiles {
    area: (u32, u32),
    contents: Vec<Box<dyn Referent>>,
    live_queue: Vec<Box<dyn Referent>>,
}

impl Tiles {
    pub fn new(length: u32, width: u32) -> Self {
        Tiles {
            area: (length, width),
            contents: Vec::<Box<dyn Referent>>::new(),
            live_queue: Vec::<Box<dyn Referent>>::new(),
        }
    }
}

// map: [[Box<dyn Referent>; 32]; 32]

pub struct Decoration {
    essence: String,
}
impl Referent for Decoration {
    fn functional_type(&self) -> FunctionalTypes {
        return FunctionalTypes::Concrete;
    }
}
