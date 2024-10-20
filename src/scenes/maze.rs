use std::collections::btree_map::Range;

use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

fn gen_oddeven(rng: &mut ThreadRng, left: usize, right: usize) -> usize {
    left + (rng.gen_range(0..=(right - left) >> 1) << 1)
}

const BLOCK_SIZE: f32 = 50.0;
const CHUNK_SIZE: usize = 32;

struct RecordChunk {
    pub record: [[u8; CHUNK_SIZE]; CHUNK_SIZE],
}

impl RecordChunk {
    pub fn new() -> Self {
        RecordChunk {
            record: [[0u8; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    /// Algorithm that recursively generates maze.
    fn generate_area(&mut self, rng: &mut ThreadRng, lx: usize, ly: usize, rx: usize, ry: usize) {
        if lx == rx || ly == ry {
            for i in lx..=rx {
                for j in ly..=ry {
                    self.record[i][j] = 0;
                }
            }
            return;
        }

        let mx = gen_oddeven(rng, lx + 1, rx - 1);
        let my = gen_oddeven(rng, ly + 1, ry - 1);

        // Add walls that divide areas.
        for i in lx..=rx {
            self.record[i][my] = 1;
        }
        for j in ly..=ry {
            self.record[mx][j] = 1;
        }

        // Recursively generate areas.
        self.generate_area(rng, lx, ly, mx - 1, my - 1);
        self.generate_area(rng, lx, my + 1, mx - 1, ry);
        self.generate_area(rng, mx + 1, ly, rx, my - 1);
        self.generate_area(rng, mx + 1, my + 1, rx, ry);

        // Add tunnels.
        let direction = rng.gen_range(0..4);
        if [0, 1, 2].contains(&direction) {
            self.record[gen_oddeven(rng, lx, mx)][my] = 0;
        }
        if [0, 1, 3].contains(&direction) {
            self.record[gen_oddeven(rng, mx, rx)][my] = 0;
        }
        if [0, 2, 3].contains(&direction) {
            self.record[mx][gen_oddeven(rng, ly, my)] = 0;
        }
        if [1, 2, 3].contains(&direction) {
            self.record[mx][gen_oddeven(rng, my, ry)] = 0;
        }
    }

    pub fn generate(&mut self, rng: &mut ThreadRng) {
        for i in 0..CHUNK_SIZE {
            self.record[i][0] = 1;
            self.record[0][i] = 1;
        }
        self.generate_area(rng, 1, 1, CHUNK_SIZE - 1, CHUNK_SIZE - 1);
        self.record[gen_oddeven(rng, 1, CHUNK_SIZE - 1)][0] = 0;
        self.record[0][gen_oddeven(rng, 1, CHUNK_SIZE - 1)] = 0;
    }
}

pub fn build_maze_chunk(mut commands: Commands, rng: &mut ThreadRng, offset_x: f32, offset_y: f32) {
    let mut chunk = RecordChunk::new();
    chunk.generate(rng);

    let square_sprite = Sprite {
        color: Color::srgb(1.0, 0.8, 0.1),
        custom_size: Some(Vec2::splat(BLOCK_SIZE)),
        ..default()
    };

    for i in 0..32 {
        for j in 0..32 {
            if chunk.record[i][j] == 0u8 {
                continue;
            }
            commands.spawn((
                SpriteBundle {
                    sprite: square_sprite.clone(),
                    transform: Transform::from_xyz(
                        (i as f32) * BLOCK_SIZE,
                        (j as f32) * BLOCK_SIZE,
                        1.0,
                    ),
                    ..default()
                },
                RigidBody::Static,
                Collider::rectangle(BLOCK_SIZE, BLOCK_SIZE),
            ));
        }
    }
}
