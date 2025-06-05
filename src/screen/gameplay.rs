use bevy::ecs::system::SystemParam;

use crate::core::audio::AudioSettings;
use crate::core::audio::music_audio;
use crate::core::mouse_position::MousePosition;
use crate::core::window::WINDOW_HEIGHT;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;

const WALL_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
const FLOOR_COLOR: Color = Color::srgb(0.3, 0.1, 0.1);
const PLAYER_COLOR: Color = Color::srgb(0.2, 0.5, 0.2);
const CROSSHAIR_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.33);
const BULLET_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

const PLAY_AREA_DIAMETER: f32 = WINDOW_HEIGHT;

const FLOOR_THICKNESS: f32 = 5.0;

const PLAYER_SIZE: Vec2 = Vec2::new(10.0, 20.0);
const STARTING_PLAYER_HEALTH: u16 = 100;

const CROSSHAIR_SIZE: Vec2 = Vec2::new(7.0, 7.0);
const CROSSHAIR_Z: f32 = 10.0;

const JUMP_FORCE: f32 = 200.0;
const MOVEMENT_ACCEL: f32 = 1000.0;
const MAX_MOVEMENT_SPEED: f32 = 100.0;
const DEFAULT_MOVEMENT_DAMPING_FACTOR: f32 = 0.92;

const DEFAULT_PLAYER_ATTACK_COOLDOWN: Duration = Duration::from_millis(650);

const BULLET_SIZE: Vec2 = Vec2::new(5.0, 5.0);
const BULLET_Z: f32 = 1.0;
const BULLET_SPEED: f32 = 1000.0;
const BULLET_DAMAGE: u16 = 10;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Gameplay.on_enter(spawn_gameplay_screen));
    app.add_systems(
        Update,
        update_crosshair_position.in_set(UpdateSystems::Update),
    );
    app.add_systems(
        Update,
        tick_attack_cooldown_timers.in_set(UpdateSystems::TickTimers),
    );
    app.add_systems(
        Update,
        handle_bullet_collisions.in_set(UpdateSystems::SyncLate),
    );

    app.configure::<(GameplayAssets, GameplayAction)>();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Crosshair;

#[derive(Component)]
struct Bullet {
    source: Entity,
    damage: u16,
}

#[derive(Component)]
struct Health(u16);

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct AttackCooldown(Timer);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(f32);

fn spawn_gameplay_screen(
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    assets: Res<GameplayAssets>,
) {
    commands.spawn((
        music_audio(&audio_settings, assets.music.clone()),
        DespawnOnExitState::<Screen>::Recursive,
    ));

    // left wall
    commands.spawn((
        Transform::from_translation(Vec3::new(-PLAY_AREA_DIAMETER, 0.0, 0.0)),
        Sprite::from_color(
            WALL_COLOR,
            Vec2::new(PLAY_AREA_DIAMETER, PLAY_AREA_DIAMETER),
        ),
        Collider::rectangle(PLAY_AREA_DIAMETER, PLAY_AREA_DIAMETER),
        RigidBody::Static,
        DespawnOnExitState::<Screen>::Recursive,
    ));

    // top wall
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, PLAY_AREA_DIAMETER, 0.0)),
        Sprite::from_color(
            WALL_COLOR,
            Vec2::new(PLAY_AREA_DIAMETER, PLAY_AREA_DIAMETER),
        ),
        Collider::rectangle(PLAY_AREA_DIAMETER, PLAY_AREA_DIAMETER),
        RigidBody::Static,
        DespawnOnExitState::<Screen>::Recursive,
    ));

    // right wall
    commands.spawn((
        Transform::from_translation(Vec3::new(PLAY_AREA_DIAMETER, 0.0, 0.0)),
        Sprite::from_color(
            WALL_COLOR,
            Vec2::new(PLAY_AREA_DIAMETER, PLAY_AREA_DIAMETER),
        ),
        Collider::rectangle(PLAY_AREA_DIAMETER, PLAY_AREA_DIAMETER),
        RigidBody::Static,
        DespawnOnExitState::<Screen>::Recursive,
    ));

    // bottom wall
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, -PLAY_AREA_DIAMETER, 0.0)),
        Sprite::from_color(
            WALL_COLOR,
            Vec2::new(PLAY_AREA_DIAMETER, PLAY_AREA_DIAMETER),
        ),
        Collider::rectangle(PLAY_AREA_DIAMETER, PLAY_AREA_DIAMETER),
        RigidBody::Static,
        DespawnOnExitState::<Screen>::Recursive,
    ));

    // floor 1
    commands.spawn((
        Transform::from_translation(Vec3::new(
            -(PLAY_AREA_DIAMETER * 0.25),
            -(PLAY_AREA_DIAMETER * 0.50),
            0.0,
        )),
        Sprite::from_color(
            FLOOR_COLOR,
            Vec2::new(PLAY_AREA_DIAMETER * 0.5, FLOOR_THICKNESS),
        ),
        Collider::rectangle(PLAY_AREA_DIAMETER * 0.5, FLOOR_THICKNESS),
        RigidBody::Static,
        DespawnOnExitState::<Screen>::Recursive,
    ));

    // floor 2
    commands.spawn((
        Transform::from_translation(Vec3::new(
            PLAY_AREA_DIAMETER * 0.25,
            -(PLAY_AREA_DIAMETER * 0.25),
            0.0,
        )),
        Sprite::from_color(
            FLOOR_COLOR,
            Vec2::new(PLAY_AREA_DIAMETER / 2.0, FLOOR_THICKNESS),
        ),
        Collider::rectangle(PLAY_AREA_DIAMETER / 2.0, FLOOR_THICKNESS),
        RigidBody::Static,
        DespawnOnExitState::<Screen>::Recursive,
    ));

    // floor 3
    commands.spawn((
        Transform::from_translation(Vec3::new(-(PLAY_AREA_DIAMETER * 0.25), 0.0, 0.0)),
        Sprite::from_color(
            FLOOR_COLOR,
            Vec2::new(PLAY_AREA_DIAMETER * 0.5, FLOOR_THICKNESS),
        ),
        Collider::rectangle(PLAY_AREA_DIAMETER * 0.5, FLOOR_THICKNESS),
        RigidBody::Static,
        DespawnOnExitState::<Screen>::Recursive,
    ));

    // floor 4
    commands.spawn((
        Transform::from_translation(Vec3::new(
            PLAY_AREA_DIAMETER * 0.25,
            PLAY_AREA_DIAMETER * 0.25,
            0.0,
        )),
        Sprite::from_color(
            FLOOR_COLOR,
            Vec2::new(PLAY_AREA_DIAMETER * 0.5, FLOOR_THICKNESS),
        ),
        Collider::rectangle(PLAY_AREA_DIAMETER * 0.5, FLOOR_THICKNESS),
        RigidBody::Static,
        DespawnOnExitState::<Screen>::Recursive,
    ));

    // player
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, -(PLAY_AREA_DIAMETER * 0.33), 0.0)),
        Sprite::from_color(PLAYER_COLOR, PLAYER_SIZE),
        Collider::rectangle(PLAYER_SIZE.x, PLAYER_SIZE.y),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
        DespawnOnExitState::<Screen>::Recursive,
        Player,
        MovementDampingFactor(DEFAULT_MOVEMENT_DAMPING_FACTOR),
        AttackCooldown(Timer::new(DEFAULT_PLAYER_ATTACK_COOLDOWN, TimerMode::Once)),
        Health(STARTING_PLAYER_HEALTH),
    ));

    // crosshair
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, CROSSHAIR_Z)),
        Sprite::from_color(CROSSHAIR_COLOR, CROSSHAIR_SIZE),
        DespawnOnExitState::<Screen>::Recursive,
        Crosshair,
    ));
}

/// Updates the position of the crosshair to match the mouse position
fn update_crosshair_position(
    mouse_position: Res<MousePosition>,
    mut crosshair_query: Query<&mut Transform, With<Crosshair>>,
) {
    let mut crosshair_transform = r!(crosshair_query.single_mut());
    crosshair_transform.translation.x = mouse_position.0.x;
    crosshair_transform.translation.y = mouse_position.0.y;
}

/// Advances all the attack cooldown timers
fn tick_attack_cooldown_timers(time: Res<Time>, attack_cooldown_query: Query<&mut AttackCooldown>) {
    for mut attack_cooldown in attack_cooldown_query {
        attack_cooldown.0.tick(time.delta());
    }
}

/// Filters collisions for bullets
#[derive(SystemParam)]
pub struct BulletCollisionHooks<'w, 's> {
    bullet_query: Query<'w, 's, &'static Bullet>,
}

// Implement the `CollisionHooks` trait.
impl CollisionHooks for BulletCollisionHooks<'_, '_> {
    #[cfg_attr(bevy_lint, allow(bevy::borrowed_reborrowable))]
    fn filter_pairs(&self, collider1: Entity, collider2: Entity, _: &mut Commands) -> bool {
        // don't allow collisions between an entity and the bullets it fires
        if let Ok(bullet) = self.bullet_query.get(collider1) {
            return bullet.source != collider2;
        }

        if let Ok(bullet) = self.bullet_query.get(collider2) {
            return bullet.source != collider1;
        }

        true
    }
}

/// Deals with collisions involving bullets
fn handle_bullet_collisions(
    mut commands: Commands,
    collisions: Collisions,
    bullet_query: Query<(Entity, &Bullet)>,
    mut damageable_query: Query<(&mut Health, Entity)>,
) {
    for (bullet_entity, bullet) in bullet_query {
        let mut hit = false;
        for other_entity in collisions.entities_colliding_with(bullet_entity) {
            if other_entity == bullet.source {
                // entities can't damage themselves
                continue;
            }

            if let Ok((mut health, _)) = damageable_query.get_mut(other_entity) {
                health.0 = health.0.saturating_sub(bullet.damage);
            }

            hit = true;
        }

        if hit {
            commands.entity(bullet_entity).despawn();
        }
    }
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameplayAssets {
    #[asset(path = "audio/music/545458__bertsz__bit-forest-evil-theme-music.ogg")]
    music: Handle<AudioSource>,
}

impl Configure for GameplayAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameplayAction {
    Pause,
    CloseMenu,
    Jump,
    MoveLeft,
    MoveRight,
    Attack,
}

impl Configure for GameplayAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::Pause, GamepadButton::Start)
                .with(Self::Pause, KeyCode::Escape)
                .with(Self::Pause, KeyCode::KeyP)
                .with(Self::CloseMenu, KeyCode::KeyP)
                .with(Self::Jump, KeyCode::Space)
                .with(Self::MoveLeft, KeyCode::KeyA)
                .with(Self::MoveRight, KeyCode::KeyD)
                .with(Self::Attack, MouseButton::Left),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            Screen::Gameplay.on_update((
                (spawn_pause_overlay, Menu::Pause.enter())
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_disabled.and(action_just_pressed(Self::Pause))),
                Menu::clear
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_enabled.and(action_just_pressed(Self::CloseMenu))),
                jump.in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::Jump)),
                move_left
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::MoveLeft)),
                move_right
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::MoveRight)),
                attack
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::Attack)),
                apply_movement_damping.in_set(UpdateSystems::Update),
            )),
        );
    }
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        widget::blocking_overlay(1),
        ThemeColor::Overlay.set::<BackgroundColor>(),
        DespawnOnExitState::<Screen>::default(),
        DespawnOnDisableState::<Menu>::default(),
    ));
}

/// Makes the player jump
fn jump(velocity_query: Query<&mut LinearVelocity, With<Player>>) {
    for mut velocity in velocity_query {
        velocity.y = JUMP_FORCE;
    }
}

/// Makes the player move to the left
fn move_left(time: Res<Time>, velocity_query: Query<&mut LinearVelocity, With<Player>>) {
    let delta_secs = time.delta_secs();
    for mut velocity in velocity_query {
        if velocity.x > -MAX_MOVEMENT_SPEED {
            velocity.x -= MOVEMENT_ACCEL * delta_secs;
        }
    }
}

/// Makes the player move to the right
fn move_right(time: Res<Time>, velocity_query: Query<&mut LinearVelocity, With<Player>>) {
    let delta_secs = time.delta_secs();
    for mut velocity in velocity_query {
        if velocity.x < MAX_MOVEMENT_SPEED {
            velocity.x += MOVEMENT_ACCEL * delta_secs;
        }
    }
}

/// Makes the player attack
fn attack(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    player_query: Query<(&Transform, &mut AttackCooldown, Entity), With<Player>>,
) {
    for (transform, mut attack_cooldown, player_entity) in player_query {
        if !attack_cooldown.0.finished() {
            continue;
        }

        let to_mouse_position = (mouse_position.0 - transform.translation.xy()).normalize();

        commands.spawn((
            Sprite::from_color(BULLET_COLOR, BULLET_SIZE),
            Transform::from_translation(transform.translation.with_z(BULLET_Z)),
            LinearVelocity(to_mouse_position * BULLET_SPEED),
            RigidBody::Dynamic,
            Collider::rectangle(BULLET_SIZE.x, BULLET_SIZE.y),
            GravityScale(0.0),
            CollisionEventsEnabled,
            Bullet {
                source: player_entity,
                damage: BULLET_DAMAGE,
            },
        ));

        attack_cooldown.0.reset();
    }
}

/// Slows down movement in the X direction.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}
