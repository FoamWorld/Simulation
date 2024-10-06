#![allow(warnings, unused)]
extern crate serde_json;

mod physics;
mod semiology;
mod thaumatology;

extern crate avian2d;
extern crate bevy;

// https://docs.rs/avian2d/latest/avian2d/index.html
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

mod fps_text;
use fps_text::FpsTextPlugin;

use physics::control::{movement, setup_actor};

#[bevy_main]
fn main() {
    let mut app = App::new();
    // app.add_resource(Msaa { samples: 4 });
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulation v0.1.0".into(),
                resolution: (800., 600.).into(),
                resizable: false,
                decorations: true,
                ..default()
            }),
            ..default()
        }),
        FpsTextPlugin,
        PhysicsPlugins::default().with_length_unit(20.0),
    ));
    app.insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)));
    app.insert_resource(Gravity(Vector::ZERO));
    app.add_systems(Startup, setup);
    app.add_systems(Startup, setup_actor);
    app.add_systems(Update, movement);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());
    let square_sprite = Sprite {
        color: Color::srgb(0.1, 0.0, 1.0),
        custom_size: Some(Vec2::splat(10.0)),
        ..default()
    };
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(-30.0, 0.0, 1.0).with_scale(Vec3::new(1.0, 11.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(10.0, 10.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(30.0, 0.0, 1.0),
            ..default()
        },
        Mass(100.0),
        Friction::new(0.4).with_dynamic_coefficient(0.6),
        RigidBody::Dynamic,
        Collider::rectangle(10.0, 10.0),
    ));
}
