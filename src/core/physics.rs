use crate::{prelude::*, screen::BulletCollisionHooks};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        PhysicsPlugins::default()
            .with_length_unit(PIXELS_PER_METER)
            .with_collision_hooks::<BulletCollisionHooks>()
            .set(PhysicsInterpolationPlugin::interpolate_all()),
    );
    app.insert_resource(Gravity(Vec2::Y * -9.81 * PIXELS_PER_METER * 3.0));

    app.add_systems(StateFlush, Pause.on_edge(unpause_physics, pause_physics));
}

const PIXELS_PER_METER: f32 = 16.0;

#[cfg_attr(feature = "native_dev", hot)]
fn unpause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.unpause();
}

#[cfg_attr(feature = "native_dev", hot)]
fn pause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.pause();
}
