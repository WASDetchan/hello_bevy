use avian3d::{math::Vector, prelude::*};
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let seat_mesh = meshes.add(Cone::new(1.0, 4.0));

    let car_translation =
        Transform::from_xyz(10.0, 1.5, 0.0).with_rotation(Quat::look_to_lh(Vec3::Y, Vec3::X));

    let front_left_wheel_translation =
        Transform::from_xyz(-2.0, 0.0, 1.0).with_rotation(Quat::look_to_lh(Vec3::Y, Vec3::Z));

    let front_right_wheel_translation =
        Transform::from_xyz(0.0, 0.0, -2.0).looking_at(Vec3::new(0.0, 1.0, -2.0), Dir3::Z);

    let base = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cone(1.0, 4.0),
            Mesh3d(seat_mesh),
            MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
            car_translation,
        ))
        .id();

    let front_wheel_mesh = meshes.add(Cylinder::new(0.5, 0.2));
    let rear_wheel_mesh = meshes.add(Cylinder::new(1.5, 0.2));

    let front_wheel_collider = Collider::cylinder(0.5, 0.2);

    let wheel_material = materials.add(Color::srgb(0.8, 0.8, 0.9));

    let front_left_wheel = commands
        .spawn((
            RigidBody::Dynamic,
            Mesh3d(front_wheel_mesh.clone()),
            MeshMaterial3d(wheel_material.clone()),
            front_wheel_collider.clone(),
            front_left_wheel_translation,
            Friction::new(1.0),
            ChildOf(base),
        ))
        .id();

    // let front_right_wheel = commands
    //     .spawn((
    //         RigidBody::Dynamic,
    //         Mesh3d(front_wheel_mesh),
    //         MeshMaterial3d(wheel_material),
    //         front_wheel_collider,
    //         front_right_wheel_translation,
    //         Friction::new(1.0),
    //         ChildOf(base),
    //     ))
    //     .id();

    // commands.spawn((RevoluteJoint::new(base, front_left_wheel).with_hinge_axis(Vec3::Z),));
    // commands.spawn((RevoluteJoint::new(base, front_right_wheel).with_hinge_axis(Vec3::Z),));

    base
}
