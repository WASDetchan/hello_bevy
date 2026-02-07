use avian3d::prelude::Position;
use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

#[derive(Component, Debug)]
pub struct FollowCamera {
    normal: Vec3,
    length: f32,
    entity: Option<Entity>,
}

impl Default for FollowCamera {
    fn default() -> Self {
        Self {
            normal: Vec3::new(0.0, 1.0, 0.0),
            length: 10.0,
            entity: None,
        }
    }
}

impl FollowCamera {
    pub fn follow(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }
    pub fn unfollow(&mut self) {
        self.entity = None;
    }
    pub fn following(&self) -> Option<Entity> {
        self.entity
    }
}

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mouse_control, update_follow, follow))
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Dir3::Y),
        FollowCamera::default(),
    ));
}

fn mouse_control(
    scroll: Res<AccumulatedMouseScroll>,
    motion: Res<AccumulatedMouseMotion>,
    cameras: Query<&mut FollowCamera>,
) {
    for mut shift in cameras {
        shift.length -= scroll.delta.y;

        shift.normal = shift
            .normal
            .rotate_y(-motion.delta.x / 400.0)
            .rotate_axis(
                shift
                    .normal
                    .cross(Vec3::new(0.0, 1.0, 0.0))
                    .try_normalize()
                    .unwrap_or(Vec3::new(1.0, 0.0, 0.0)),
                motion.delta.y / 100.0,
            )
            .normalize();
    }
}

fn follow(follow: Query<&Position>, camera: Query<(&Camera3d, &mut Transform, &FollowCamera)>) {
    for (_, mut transform, shift) in camera {
        let Some(entity) = shift.entity else {
            continue;
        };
        let Ok(&follow) = follow.get(entity) else {
            return;
        };
        let follow_pos = follow.0;

        let camera_pos = follow_pos + shift.normal * shift.length;

        transform.translation = camera_pos;
        transform.look_at(follow_pos, Dir3::Y);
    }
}

#[derive(Resource)]
pub struct GlobalFollow(pub Option<Entity>);

fn update_follow(camera: Query<&mut FollowCamera>, entity: Option<Res<GlobalFollow>>) {
    let Some(entity) = (if let Some(v) = entity {
        v.0
    } else {
        return;
    }) else {
        return;
    };

    for mut camera in camera {
        camera.follow(entity);
    }
}
