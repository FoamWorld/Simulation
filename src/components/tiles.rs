use std::rc::Rc;

use crate::semiology::referent::Referent;

pub struct Tiles {
	area: (u32, u32),
	contents: Vec<Box<dyn Referent>>,
	live_queue: Vec<Box<dyn Referent>>
}

// map: [[Box<dyn Referent>; 32]; 32]
