use crate::semiology::referent::{Referent, Void};
use std::{cell::RefCell, rc::Rc};

trait Generator {
    fn generate(&mut self) -> &dyn Referent;
}

struct GeneratorTerminus {}
impl Generator for GeneratorTerminus {
    fn generate(&mut self) -> &dyn Referent {
        return &Void {};
    }
}

pub struct Passage {
    pub locked: Option<u32>,
    pub link_src: Rc<RefCell<dyn Referent>>,
    pub link_dest: Rc<RefCell<dyn Referent>>,
}
impl Passage {
    fn pass(&mut self) {
    }
}
impl Referent for Passage {
    fn is_interactive(&self) -> bool {
        return true;
    }
}
