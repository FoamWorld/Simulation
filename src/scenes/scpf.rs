use crate::components::character::Character;
use crate::components::entity::{Humanoid, NumericEntity};
use crate::components::memory::Memory;
use crate::components::passage::{passage_portal, Generator, Passage};
use crate::components::tiles::{self, Tiles};
use crate::semiology::referent::{Barrier, Referent};
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
    let walls = Rc::new(RefCell::new(Barrier {
        level: barrier_level,
        transparent: false,
    }));
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

fn build_std_containment_room(link_src: Rc<RefCell<dyn Referent>>) -> Rc<RefCell<Tiles>> {
    let tiles_rc = Rc::new(RefCell::new(build_walled(30, 30, 0xa0u8)));
    let mut tiles = tiles_rc.borrow_mut();
    tiles.insert(
        (1, 20, 28, 20),
        Barrier {
            level: 0x7fu8,
            transparent: true,
        },
    );
    tiles.set(
        23,
        29,
        Rc::new(RefCell::new(passage_portal(tiles_rc.clone(), link_src))),
    );
    return tiles_rc.clone();
}

fn create_entity_rat() -> NumericEntity {
    NumericEntity::new("rat".to_string(), 100.0, 10.0, 11.0)
}
// create_entity_encoded_073_traditional

fn build_random_containment_room(parent: Rc<RefCell<dyn Referent>>) -> Rc<RefCell<Tiles>> {
    let tiles_rc = build_std_containment_room(parent);
    tiles_rc
        .borrow_mut()
        .set(10, 10, Rc::new(RefCell::new(create_entity_rat())));
    return tiles_rc;
}

fn build_random_containment_corridor(
    left: Option<Rc<RefCell<dyn Referent>>>,
    right: Option<Rc<RefCell<dyn Referent>>>,
) -> Rc<RefCell<dyn Referent>> {
    let tiles_rc = Rc::new(RefCell::new(build_walled(120, 9, 0x70u8)));
    for i in 1..5 {
        tiles_rc.borrow_mut().set(
            i * 24,
            0,
            Rc::new(RefCell::new(Passage::new(
                tiles_rc.clone(),
                Generator::new(|| build_random_containment_room(tiles_rc.clone())),
            ))),
        );
    }
    match left {
        Some(x) => {
            tiles_rc.borrow_mut().set(
                0,
                4,
                Rc::new(RefCell::new(passage_portal(tiles_rc.clone(), x))),
            );
        }
        None => {
            tiles_rc.borrow_mut().set(
                0,
                4,
                Rc::new(RefCell::new(Passage::new(
                    tiles_rc.clone(),
                    Generator::new(|| {
                        build_random_containment_corridor(None, Some(tiles_rc.clone()))
                    }),
                ))),
            );
        }
    }
    match right {
        Some(x) => {
            tiles_rc.borrow_mut().set(
                0,
                119,
                Rc::new(RefCell::new(passage_portal(tiles_rc.clone(), x))),
            );
        }
        None => {
            tiles_rc.borrow_mut().set(
                0,
                119,
                Rc::new(RefCell::new(Passage::new(
                    tiles_rc.clone(),
                    Generator::new(|| {
                        build_random_containment_corridor(Some(tiles_rc.clone()), None)
                    }),
                ))),
            );
        }
    }
    return tiles_rc;
}
