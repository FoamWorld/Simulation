use super::rune::{EveFlux, Rune};
use std::cell::RefCell;

struct CircleSrc {
    nodes: Vec<Box<RefCell<dyn Rune>>>,
    edges: Vec<Fuse>,
}

struct Fuse {
    length: u64,
    resist: u64,
    radiate: u64,
    collected: u64,
}
impl Fuse {
    fn spark(&mut self, flux: u64) {}
}
