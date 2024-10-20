#![allow(warnings, unused)]
extern crate rand;
extern crate serde_json;

mod physics;
mod semiology;
mod thaumatology;

extern crate avian2d;
extern crate bevy;

// https://docs.rs/avian2d/latest/avian2d/index.html
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

mod constants;
use constants::*;

mod fps_text;
use fps_text::FpsTextPlugin;

use physics::control::*;

mod scenes;
use scenes::maze::build_maze_chunk;

#[bevy_main]
fn main() {
    let mut app = App::new();
    // app.add_resource(Msaa { samples: 4 });
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulation v0.1.0".into(),
                resolution: WINDOW_AREA.into(),
                resizable: false,
                decorations: true,
                ..default()
            }),
            ..default()
        }),
        FpsTextPlugin,
        PhysicsPlugins::default().with_length_unit(PHYSCIS_UNIT),
    ));
    app.insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)));
    app.insert_resource(Gravity(Vector::ZERO));
    app.init_resource::<CursorCoords>();
    app.add_systems(Startup, setup);
    app.add_systems(Startup, setup_actor);
    app.add_systems(Update, (inputs_move, update_camera).chain());
    app.add_systems(Update, (translate_cursor_position, inputs_move, inputs_q));
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    let square_sprite = Sprite {
        color: Color::srgb(0.1, 0.0, 1.0),
        custom_size: Some(Vec2::splat(10.0)),
        ..default()
    };
    build_maze_chunk(commands, &mut rand::thread_rng(), 20.0, 20.0);
}
