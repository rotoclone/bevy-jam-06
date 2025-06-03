use crate::core::audio::AudioSettings;
use crate::core::audio::music_audio;
use crate::core::window::WINDOW_HEIGHT;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;

const WALL_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
const FLOOR_COLOR: Color = Color::srgb(0.3, 0.1, 0.1);
const PLAYER_COLOR: Color = Color::srgb(0.2, 0.5, 0.2);

const PLAY_AREA_DIAMETER: f32 = WINDOW_HEIGHT;
const FLOOR_THICKNESS: f32 = 5.0;
const PLAYER_WIDTH: f32 = 10.0;
const PLAYER_HEIGHT: f32 = 20.0;

const JUMP_FORCE: f32 = 200.0;
const MOVEMENT_ACCEL: f32 = 1000.0;
const MAX_MOVEMENT_SPEED: f32 = 100.0;
const DEFAULT_MOVEMENT_DAMPING_FACTOR: f32 = 0.92;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Gameplay.on_enter(spawn_gameplay_screen));

    app.configure::<(GameplayAssets, GameplayAction)>();
}

#[derive(Component)]
struct Player;

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
        Sprite::from_color(PLAYER_COLOR, Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
        Collider::rectangle(PLAYER_WIDTH, PLAYER_HEIGHT),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        DespawnOnExitState::<Screen>::Recursive,
        Player,
        MovementDampingFactor(DEFAULT_MOVEMENT_DAMPING_FACTOR),
    ));
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
                .with(Self::MoveRight, KeyCode::KeyD),
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

/// Slows down movement in the X direction.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}
