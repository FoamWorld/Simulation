use avian2d::{math::*, prelude::*};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
pub struct MovementSpeed(Scalar);

#[derive(Resource, Default)]
pub struct CursorCoords(Option<Vec2>);

#[derive(Component)]
pub struct MainCamera;

pub fn setup_actor(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let actor_size = Vector::new(20.0, 20.0);
    let actor_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::from_size(actor_size.f32())).into(),
        material: materials.add(Color::srgb(0.2, 0.7, 0.9)),
        ..default()
    };

    commands.spawn((
        actor_mesh.clone(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        Collider::rectangle(actor_size.x, actor_size.y),
        MovementSpeed(100.0),
        Actor,
    ));
}

pub fn translate_cursor_position(
    mut coords: ResMut<CursorCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    coords.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}

pub fn keyboard_inputs(
    mut commands: Commands,
    mut coords: ResMut<CursorCoords>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut actors: Query<(&mut LinearVelocity, &MovementSpeed), With<Actor>>,
) {
    for (mut linear_velocity, movement_speed) in &mut actors {
        let xneg = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let xpos = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
        let yneg = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
        let ypos = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
        linear_velocity.x = (xpos as i8 - xneg as i8) as Scalar * movement_speed.0;
        linear_velocity.y = (ypos as i8 - yneg as i8) as Scalar * movement_speed.0;
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            match coords.0 {
                Some(pos) => {
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(1.0, 0.0, 0.0),
                                custom_size: Some(Vec2::splat(10.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(pos.x, pos.y, 1.0),
                            ..default()
                        },
                        RigidBody::Dynamic,
                        Collider::circle(5.0),
                    ));
                }
                None => break,
            }
        }
    }
}
