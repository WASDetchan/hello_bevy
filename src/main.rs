pub mod ball;
pub mod camera;
pub mod car;

use avian3d::{
    PhysicsPlugins,
    prelude::{
        AngularVelocity, Collider, Forces, Gravity, LinearVelocity, Mass, Position, Restitution,
        RigidBody, RigidBodyForces,
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
        .insert_resource(Gravity(Vec3::default()))
        .add_systems(Startup, (setup,))
        // .add_systems(Update, ())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut follow: ResMut<GlobalFollow>,
) {
    follow.0 = Some(car::setup(commands.reborrow(), &mut meshes, &mut materials));

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
        MeshMaterial3d(material_handle.clone()),
        Restitution::new(1.0),
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(1.0, 100.0),
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
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
