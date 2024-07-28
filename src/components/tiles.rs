use crate::semiology::referent::{FunctionalTypes, Referent};
use std::{cell::RefCell, rc::Rc};

pub struct Tiles {
    area: (u32, u32),
    contents: Vec<Rc<RefCell<dyn Referent>>>,
    live_queue: Vec<Rc<dyn Referent>>,
}

impl Tiles {
    pub fn new(length: u32, width: u32) -> Self {
        Tiles {
            area: (length, width),
            contents: Vec::<Rc<RefCell<dyn Referent>>>::with_capacity((length * width) as usize),
            live_queue: Vec::<Rc<dyn Referent>>::new(),
        }
    }
    pub fn get(&self, i: u32, j: u32) -> Rc<RefCell<(dyn Referent + 'static)>> {
        let ind = (i * self.area.0 + j) as usize;
        return self.contents[ind].clone();
    }
    pub fn insert<T>(&mut self, area: (u32, u32, u32, u32), tile: T)
    where
        T: Referent + 'static,
    {
        let rc = Rc::new(RefCell::<T>::new(tile));
        for i in area.0..=area.0 + area.2 {
            for j in area.1..=area.1 + area.3 {
                let ind = (i * self.area.0 + j) as usize;
                self.contents[ind] = rc.clone();
            }
        }
    }
}

// map: [[Box<dyn Referent>; 32]; 32]

pub struct Decoration {
    pub essence: String,
}
impl Referent for Decoration {
    fn functional_type(&self) -> FunctionalTypes {
        return FunctionalTypes::Concrete;
    }
}
