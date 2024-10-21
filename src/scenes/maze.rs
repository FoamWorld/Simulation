use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

fn gen_oddeven(rng: &mut ThreadRng, left: usize, right: usize) -> usize {
    left + (rng.gen_range(0..=(right - left) >> 1) << 1)
}

const BLOCK_SIZE: f32 = 50.0;
const CHUNK_SIZE: usize = 32;

struct RecordChunk {
    walls: Vec<((usize, usize), (usize, usize))>,
    // pub record: [[u8; CHUNK_SIZE]; CHUNK_SIZE],
}

impl RecordChunk {
    pub fn new() -> Self {
        RecordChunk {
            walls: Vec::<((usize, usize), (usize, usize))>::new(),
        }
    }

    pub fn put(&mut self, x: usize, y: usize) {
        self.walls.push(((x, x), (y, y)));
    }

    pub fn iter(&self) -> std::slice::Iter<((usize, usize), (usize, usize))> {
        return self.walls.iter();
    }

    pub fn push(&mut self, x: (usize, usize), y: (usize, usize)) {
        self.walls.push((x, y));
    }

    pub fn push_with_split_col(&mut self, x: usize, y: (usize, usize, usize)) {
        let (ly, my, ry) = y;
        if ly != my {
            self.walls.push(((x, x), (ly, my - 1)))
        };
        if my != ry {
            self.walls.push(((x, x), (my + 1, ry)))
        };
    }

    pub fn push_with_split_lin(&mut self, x: (usize, usize, usize), y: usize) {
        let (lx, mx, rx) = x;
        if lx != mx {
            self.walls.push(((lx, mx - 1), (y, y)))
        };
        if mx != rx {
            self.walls.push(((mx + 1, rx), (y, y)))
        };
    }

    /// Algorithm that recursively generates maze.
    fn generate_area(&mut self, rng: &mut ThreadRng, lx: usize, ly: usize, rx: usize, ry: usize) {
        if lx == rx || ly == ry {
            return;
        }

        let mx = gen_oddeven(rng, lx + 1, rx - 1);
        let my = gen_oddeven(rng, ly + 1, ry - 1);

        // Add walls that divide areas (with tunnels).
        self.put(mx, my);
        let direction = rng.gen_range(0..4);
        if direction != 0 {
            let tunnel_lx = gen_oddeven(rng, lx, mx - 1);
            self.push_with_split_lin((lx, tunnel_lx, mx - 1), my);
        } else {
            self.push((lx, mx - 1), (my, my));
        }
        if direction != 1 {
            let tunnel_rx = gen_oddeven(rng, mx + 1, rx);
            self.push_with_split_lin((mx + 1, tunnel_rx, rx), my);
        } else {
            self.push((mx + 1, rx), (my, my));
        }
        if direction != 2 {
            let tunnel_ly = gen_oddeven(rng, ly, my - 1);
            self.push_with_split_col(mx, (ly, tunnel_ly, my - 1));
        } else {
            self.push((mx, mx), (ly, my - 1));
        }
        if direction != 3 {
            let tunnel_ry = gen_oddeven(rng, my + 1, ry);
            self.push_with_split_col(mx, (my + 1, tunnel_ry, ry));
        } else {
            self.push((mx, mx), (my + 1, ry));
        }

        // Recursively generate areas.
        self.generate_area(rng, lx, ly, mx - 1, my - 1);
        self.generate_area(rng, lx, my + 1, mx - 1, ry);
        self.generate_area(rng, mx + 1, ly, rx, my - 1);
        self.generate_area(rng, mx + 1, my + 1, rx, ry);
    }

    pub fn generate(&mut self, rng: &mut ThreadRng) {
        self.put(0, 0);
        self.push_with_split_col(0, (1, gen_oddeven(rng, 1, CHUNK_SIZE - 1), CHUNK_SIZE - 1));
        self.push_with_split_lin((1, gen_oddeven(rng, 1, CHUNK_SIZE - 1), CHUNK_SIZE - 1), 0);
        self.generate_area(rng, 1, 1, CHUNK_SIZE - 1, CHUNK_SIZE - 1);
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

    for (range_x, range_y) in chunk.iter() {
        let (lx, rx): (usize, usize) = *range_x;
        let (ly, ry): (usize, usize) = *range_y;
        // if lx == rx && ly == ry { continue; }
        commands.spawn((
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_xyz(
                    ((lx + rx) as f32) * 0.5 * BLOCK_SIZE + offset_x,
                    ((ly + ry) as f32) * 0.5 * BLOCK_SIZE + offset_y,
                    1.0,
                )
                .with_scale(Vec3::new(
                    (rx - lx + 1) as f32,
                    (ry - ly + 1) as f32,
                    1.0,
                )),
                ..default()
            },
            RigidBody::Static,
            Collider::rectangle(BLOCK_SIZE, BLOCK_SIZE),
        ));
    }
}
