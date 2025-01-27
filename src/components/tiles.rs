use crate::semiology::referent::{Referent, Void};
use std::{cell::RefCell, rc::Rc, usize};

pub struct Tiles {
    area: (u32, u32),
    contents: Vec<Rc<RefCell<dyn Referent>>>,
    live_queue: Vec<Rc<RefCell<dyn Referent>>>,
}

impl Tiles {
    pub fn new(length: u32, width: u32) -> Self {
        let mut vec = Vec::<Rc<RefCell<dyn Referent>>>::new();
        vec.resize((length * width) as usize, Rc::new(RefCell::new(Void {})));
        Tiles {
            area: (length, width),
            contents: vec,
            live_queue: Vec::<Rc<RefCell<dyn Referent>>>::new(),
        }
    }
    pub fn get(&self, i: u32, j: u32) -> Rc<RefCell<(dyn Referent + 'static)>> {
        let ind = (i * self.area.0 + j) as usize;
        return self.contents[ind].clone();
    }
    pub fn set<T>(&mut self, i: u32, j: u32, rc: Rc<RefCell<T>>)
    where
        T: Referent + 'static,
    {
        let ind = (i * self.area.0 + j) as usize;
        self.contents[ind] = rc;
    }
    pub fn insert<T>(&mut self, area: (u32, u32, u32, u32), tile: T)
    where
        T: Referent + 'static,
    {
        let rc = Rc::new(RefCell::<T>::new(tile));
        for i in area.0..=area.2 {
            for j in area.1..=area.3 {
                let ind = (i * self.area.0 + j) as usize;
                self.contents[ind] = rc.clone();
            }
        }
    }
}
impl Referent for Tiles {}

// map: [[Box<dyn Referent>; 32]; 32]
