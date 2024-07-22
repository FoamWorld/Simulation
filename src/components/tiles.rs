use crate::semiology::referent::Referent;

pub struct Tiles {
	area: (u32, u32),
	contents: Vec<Box<dyn Referent>>,
	live_queue: Vec<Box<dyn Referent>>
}

impl Tiles {
	pub fn new(length: u32, width: u32) -> Self{
		Tiles {
			area: (length, width),
			contents: Vec::<Box::<dyn Referent>>::new(),
			live_queue: Vec::<Box::<dyn Referent>>::new()
		}
	}
}

// map: [[Box<dyn Referent>; 32]; 32]
