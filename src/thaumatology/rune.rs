use bevy_ecs::{component::Component, entity::Entity};

pub struct EveFlux {
    flux: f32,
}

#[derive(Component)]
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

pub trait Rune {
    fn activate(&self, flux: &mut EveFlux, istream: ComposedConcept) -> ComposedConcept;
    // fn next(&self, from: f32, edges: Vec<u32>) {}
}

struct ChargerSimple {
    speed: f32,
}
impl Rune for ChargerSimple {
    fn activate(&self, flux: &mut EveFlux, _: ComposedConcept) -> ComposedConcept {
        flux.flux += self.speed;
        return ComposedConcept::new();
    }
}

struct ChargerConsumption {
    laid: Entity,
}
