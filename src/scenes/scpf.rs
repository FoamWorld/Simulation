use crate::components::character::Character;
use crate::components::entity::Humanoid;
use crate::components::memory::Memory;
use crate::components::tiles::Tiles;
use crate::semiology::referent::Barrier;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::Rng;
use std::any::Any;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

fn build_scpf_researcher<R: Rng>(rng: &mut R, authority_level: String) -> Humanoid {
    let uniform = Uniform::new(0x80u8, 0xffu8);
    let character = Character::from_code(
        "INNN".to_string(),
        [
            uniform.sample(rng),
            uniform.sample(rng),
            uniform.sample(rng),
            uniform.sample(rng),
        ],
    );
    let mut memory = Memory::new();
    memory.learn("basic".to_string(), 0xef);
    memory.learn("language-general".to_string(), 0xed);
    memory.learn("skill-usage-pill_w".to_string(), 0xdf);
    memory.learn("skill-usage-pill_x".to_string(), 0xcf);
    let mut relations = BTreeMap::<String, String>::new();
    relations.insert("scpf".to_string(), authority_level);
    Humanoid {
        properties: BTreeMap::<String, Box<dyn Any>>::new(),
        dmg_ratio: BTreeMap::<String, f32>::new(),
        character: character,
        memory: memory,
        relations: relations,
    }
}

fn build_scpf_o5() {}

fn build_walled(length: u32, width: u32, barrier_level: u8) -> Tiles {
    let mut tiles = Tiles::new(length, width);
    let walls = Rc::new(RefCell::new(Barrier { level: barrier_level }));
    for i in 0..length {
        tiles.set(i, 0, walls.clone());
        tiles.set(i, width - 1, walls.clone());
    }
    for j in 1..width - 1 {
        tiles.set(0, j, walls.clone());
        tiles.set(length - 1, j, walls.clone());
    }
    return tiles;
}

fn build_site_hall() {
    let mut tiles = build_walled(80, 60, 0x7fu8);
}

fn build_site_default() {}
