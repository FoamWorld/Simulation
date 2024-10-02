use super::rune::{ComposedConcept, EveFlux, Rune};
use std::cell::{Ref, RefCell};

struct CircleSrc {
    nodes: Vec<Rune>,
    nexts: Vec<Vec<usize>>,
    edges: Vec<Vec<Fuse>>,
}
impl CircleSrc {
    pub fn insert_node(self: &mut Self, node: Rune) {
        self.nodes.push(node);
    }
    pub fn insert_edge(self: &mut Self, from: usize, to: usize, edge: Fuse) {
        self.nexts[from].push(to);
        self.edges[from].push(edge);
    }
    pub fn start(self: &mut Self, from: usize, flux: &mut EveFlux, istream: &mut ComposedConcept) {
        self.nodes[from].activate(flux, istream);
        let len = self.nexts[from].len();
        for i in 0..len {
            let mut fuse = self.edges[from][i];
            let mut branch_flux = EveFlux {
                flux: flux.flux / (len as f32),
            };
            fuse.spark(&mut branch_flux);
            let next = self.nexts[from][i];
            self.start(next, &mut branch_flux, &mut istream.clone());
        }
    }
}

#[derive(Clone, Copy)]
struct Fuse {
    length: f32,
    resist: f32,
    radiate: f32,
    collected: f32,
}
impl Fuse {
    fn spark(&mut self, flux: &mut EveFlux) {
        let delta = self.radiate * self.collected - self.resist;
        flux.flux += delta;
        self.collected -= delta;
    }
}
