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
    locked: Option<u32>,
    link_src: Rc<RefCell<dyn Referent>>,
    link_dest: Rc<RefCell<dyn Referent>>,
}

impl Referent for Passage {
    fn is_interactive(&self) -> bool {
        return true;
    }
}
