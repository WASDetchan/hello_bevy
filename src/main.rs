pub mod camera;

use avian3d::{
    PhysicsPlugins,
    prelude::{
        AngularVelocity, Collider, Forces, LinearVelocity, Mass, Position, Restitution, RigidBody,
        RigidBodyForces,
    },
};
use bevy::prelude::*;

use crate::camera::{FollowCameraPlugin, GlobalFollow};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        FollowCameraPlugin,
    ));
    app.insert_resource(ClearColor(Color::srgb(0.0, 0.4, 0.0)))
        .insert_resource(GlobalFollow(None))
        .add_systems(Startup, (setup,))
        .add_systems(
            Update,
            (move_ball_keyboard_1, move_ball_keyboard_2, carlsen_effect),
        )
        .run();
}

#[derive(Component)]
struct WithCarlsenEffect;

#[derive(Component)]
struct Ball;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut follow: ResMut<GlobalFollow>,
) {
    let ball = commands.spawn((
        RigidBody::Dynamic,
        Collider::sphere(0.15),
        Mesh3d(meshes.add(Sphere::new(0.15))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        WithCarlsenEffect,
        Restitution::new(0.5),
        Transform::from_xyz(-5.0, 2.0, 0.0),
        LinearVelocity(Vec3::new(1.0, 0.0, 0.0)),
        AngularVelocity(Vec3::new(0.0, 1.0, 0.0)),
        Mass(0.2),
        Ball,
    ));

    follow.0 = Some(ball.id());

    let texture_handle = asset_server.load("plot.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });

    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(100.0, 0.1),
        Transform::from_xyz(0.0, -4.0, 0.0),
        Mesh3d(meshes.add(Cylinder::new(100.0, 0.1))),
        MeshMaterial3d(material_handle),
        Restitution::new(1.0),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn carlsen_effect(query: Query<(&WithCarlsenEffect, Forces)>, mut gizmos: Gizmos) {
    for (_, mut forces) in query {
        let v = forces.linear_velocity();
        let omega = forces.angular_velocity();
        let f = 0.01 * omega.cross(v);
        forces.apply_force(f);

        let ball_pos = forces.position().0;
        let acc = forces.accumulated_linear_acceleration() / 20.0;
        gizmos.arrow(ball_pos, ball_pos + acc, Color::linear_rgb(1.0, 0.0, 1.0));
    }
}

fn move_ball_keyboard_1(
    query: Query<(&Ball, Forces)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut gizmos: Gizmos,
) {
    let a = 40.0;

    for (_, mut forces) in query {
        let normal_vel = forces
            .linear_velocity()
            .with_y(0.0)
            .try_normalize()
            .unwrap_or(Vec3::new(1.0, 0.0, 0.0));

        let tangent_vel = normal_vel.cross(*Dir3::Y).normalize();

        if keyboard_input.pressed(KeyCode::KeyA) {
            forces.apply_linear_acceleration(-a * tangent_vel);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            forces.apply_linear_acceleration(a * tangent_vel);
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            forces.apply_linear_acceleration(a * normal_vel);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            forces.apply_linear_acceleration(-a * normal_vel);
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            forces.apply_linear_impulse(Vec3::new(0.0, 1.0, 0.0));
        }

        let ball_pos = forces.position().0;
        let vel = forces.linear_velocity() / 20.0;
        let ang = forces.angular_velocity() / 20.0;
        let acc = forces.accumulated_linear_acceleration() / 20.0;
        gizmos.arrow(ball_pos, ball_pos + vel, Color::linear_rgb(0.0, 0.0, 1.0));
        gizmos.arrow(ball_pos, ball_pos + acc, Color::linear_rgb(1.0, 0.0, 0.0));
        gizmos.arrow(ball_pos, ball_pos + ang, Color::linear_rgb(1.0, 1.0, 0.0));
    }
}
fn move_ball_keyboard_2(
    query: Query<(
        &Ball,
        &mut Position,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (_, mut pos, mut linv, mut angv) in query {
        if keyboard_input.pressed(KeyCode::KeyC) {
            pos.0 = Vec3::new(0.0, 0.0, 0.0);
            linv.0 = Vec3::new(0.0, 0.0, 0.0);
            angv.0 = Vec3::new(0.0, 0.0, 0.0);
        }
    }
}
