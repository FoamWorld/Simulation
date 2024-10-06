fn create_wall(x: Scalar, y: Scalar, xl: Scalar, yl: Scalar) {
    (
        RigidBody::Static,
        Collider::rectangle(xl, yl),
        TransformBundle::from_transform(Transform::from_xyz(x, y, 0.0)),
    )
}
