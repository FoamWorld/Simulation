use bevy_ecs::{component::Component, entity::Entity};
use std::cell::RefCell;

pub struct EveFlux {
    pub flux: f32,
}

#[derive(Component, Clone)]
pub struct ComposedConcept {
    pool: Vec<String>,
}
impl ComposedConcept {
    pub fn new() -> Self {
        ComposedConcept {
            pool: Vec::<String>::new(),
        }
    }
}

pub struct Rune {
    device: Box<RefCell<dyn ThaumDevice>>,
}
impl Rune {
    pub fn activate(&self, flux: &mut EveFlux, istream: &mut ComposedConcept) {
        self.device.borrow_mut().activate(flux, istream);
    }
}

pub trait ThaumDevice {
    fn activate(&self, flux: &mut EveFlux, istream: &mut ComposedConcept);
}

struct ChargerSimple {
    speed: f32,
}
impl ThaumDevice for ChargerSimple {
    fn activate(&self, flux: &mut EveFlux, istream: &mut ComposedConcept) {
        flux.flux += self.speed;
    }
}

struct ChargerConsumption {
    laid: Entity,
}
