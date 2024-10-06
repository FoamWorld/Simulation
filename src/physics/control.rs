#[derive(Component)]
struct Actor;

fn setup_actor(
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
        Actor,
    ));
}
