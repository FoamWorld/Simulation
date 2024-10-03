use crate::semiology::composed::ComposedConcept;
use bevy_ecs::{component::Component, entity::Entity};
use std::{cell::RefCell, rc::Rc};

pub struct EveFlux {
    binding: Rc<RefCell<Entity>>,
    pub flux: f32,
}
impl EveFlux {
    pub fn copy_with(self: &Self, flux: f32) -> Self {
        EveFlux {
            binding: self.binding.clone(),
            flux,
        }
    }
}

pub struct Rune {
    device: Box<RefCell<dyn ThaumDevice>>,
}
impl Rune {
    pub fn activate(&self, flux: &mut EveFlux, istream: &ComposedConcept) -> ComposedConcept {
        return self.device.borrow_mut().activate(flux, istream);
    }
}

pub trait ThaumDevice {
    fn activate(&self, flux: &mut EveFlux, istream: &ComposedConcept) -> ComposedConcept;
}

struct ChargerSimple {
    speed: f32,
}
impl ThaumDevice for ChargerSimple {
    fn activate(&self, flux: &mut EveFlux, istream: &ComposedConcept) -> ComposedConcept {
        flux.flux += self.speed;
        return ComposedConcept::new();
    }
}

struct ChargerConsumption {
    laid: Entity,
}

struct NamedSymbol {
    name: String,
}
