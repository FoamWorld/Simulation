use crate::components::character::Character;
use crate::components::entity::Humanoid;
use crate::components::memory::Memory;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::Rng;
use std::collections::BTreeMap;

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
