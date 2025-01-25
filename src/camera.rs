use std::ops::Range;

use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*, render::camera::ScalingMode};

pub struct GameCameraPlugin;

const STARTING_SPEED: f32 = 500.;
const SLOW_DOWN_FACTOR: f32 = 2.0;
const CAMERA_MOVEMENT_EASE_OUT_SECS: f32 = 0.05;

#[derive(Component)]
pub struct TileOutline;

#[derive(Component)]
struct CameraSpeed(Vec3, Timer);

#[derive(Component)]
pub struct EditorCamera;

#[derive(Debug, Resource)]
pub struct CameraSettings {
    /// The height of the viewport in world units when the orthographic camera's scale is 1
    pub orthographic_viewport_height: f32,
    /// Clamp the orthographic camera's scale to this range
    pub orthographic_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the orthographic camera
    pub orthographic_zoom_speed: f32,

    pub movement_ease_out_time: f32,
    pub movement_max_speed: f32,
}

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings {
            orthographic_viewport_height: 100.,
            // In orthographic projections, we specify camera scale relative to a default value of 1,
            // in which one unit in world space corresponds to one pixel.
            orthographic_zoom_range: 0.1..10.0,
            // This value was hand-tuned to ensure that zooming in and out feels smooth but not slow.
            orthographic_zoom_speed: 0.2,
            movement_ease_out_time: CAMERA_MOVEMENT_EASE_OUT_SECS,
            movement_max_speed: STARTING_SPEED,
        })
        .add_systems(Startup, camera_setup)
        .add_systems(Update, (zoom, camera_movement))
        .add_systems(FixedUpdate, move_outline);
    }
}

fn camera_setup(
    mut commands: Commands,
    camera_settings: Res<CameraSettings>,
    asset_server: Res<AssetServer>,
) {
    let outline_handle = asset_server.load("outline.png");
    commands.spawn((
        TileOutline,
        Sprite::from_image(outline_handle.clone()),
        Transform::IDENTITY,
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 2,
            ..Default::default()
        },
        EditorCamera,
        Msaa::Off, // Fixes artifacs on zoom between the tiles
        Projection::Orthographic(OrthographicProjection {
            // We can set the scaling mode to FixedVertical to keep the viewport height constant as its aspect ratio changes.
            // The viewport height is the height of the camera's view in world units when the scale is 1.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: camera_settings.orthographic_viewport_height,
            },
            // This is the default value for scale for orthographic projections.
            // To zoom in and out, change this value, rather than `ScalingMode` or the camera's position.
            scale: 1.,
            ..OrthographicProjection::default_2d()
        }),
        CameraSpeed(Vec3::ZERO, Timer::from_seconds(0.0, TimerMode::Once)),
        Transform::from_xyz(0.0, 0.0, -5.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_translation(Vec3::new(300., -300., 0.)),
    ));
}

fn camera_movement(
    time: Res<Time>,
    camera_settings: Res<CameraSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    query: Single<(&Camera2d, &mut Camera, &mut CameraSpeed, &mut Transform), With<EditorCamera>>,
) {
    if keys.just_released(KeyCode::KeyE) {
        let mut camera = query.into_inner().1;
        camera.is_active = !camera.is_active;
        return;
    }

    let delta = time.delta_secs();
    let (_, camera, mut current_speed, mut transform) = query.into_inner();

    if !camera.is_active {
        return;
    }
    current_speed.1.tick(time.delta());

    let mut movement_vec = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        movement_vec.y += 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement_vec.y += -1.;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement_vec.x += 1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement_vec.x += -1.;
    }

    // No keys pressed so we start reducing speed
    if movement_vec == Vec3::ZERO {
        if current_speed.1.finished() {
            return;
        }
        let t = current_speed.1.elapsed_secs() / camera_settings.movement_ease_out_time;
        let besier = bezier_ease_out(t);

        current_speed.0 = current_speed.0 - current_speed.0.normalize() * SLOW_DOWN_FACTOR * besier;
        transform.translation += current_speed.0 * delta;
        return;
    }

    current_speed.0 = movement_vec.normalize() * camera_settings.movement_max_speed;
    transform.translation += current_speed.0 * delta;
    current_speed.1 = Timer::from_seconds(camera_settings.movement_ease_out_time, TimerMode::Once);
}

fn bezier_ease_out(t: f32) -> f32 {
    static P0: f32 = 0.19;
    static P1: f32 = 1.00;
    static P2: f32 = 0.2;
    static P3: f32 = 1.00;
    3. * (1.0 - t) * (1.0 - t) * (P1 - P0) + 6. * (1.0 - t) * (P2 - P1) + 3. * t * t * (P3 - P2)
}

fn zoom(
    camera: Single<&mut Projection, With<EditorCamera>>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    // Usually, you won't need to handle both types of projection,
    // but doing so makes for a more complete example.
    match *camera.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.orthographic_zoom_speed;
            // When changing scales, logarithmic changes are more intuitive.
            // To get this effect, we add 1 to the delta, so that a delta of 0
            // results in no multiplicative effect, positive values result in a multiplicative increase,
            // and negative values result in multiplicative decreases.
            let multiplicative_zoom = 1. + delta_zoom;

            orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(
                camera_settings.orthographic_zoom_range.start,
                camera_settings.orthographic_zoom_range.end,
            );
        }
        _ => unreachable!(),
    }
}

pub fn move_outline(
    query: Single<&mut Transform, With<TileOutline>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<EditorCamera>>,
    sprites: Query<&Transform, (With<Sprite>, Without<TileOutline>)>,
) {
    let mut outline = query.into_inner();
    let window = windows.get_single();

    if window.is_err() {
        return;
    }
    let window = window.unwrap();

    let (camera, position) = cameras.single();
    if !camera.is_active {
        outline.translation.z = -1.;
        return;
    }
    if let Some(Ok(world_position)) = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(position, cursor))
        .map(|ray| ray.map(|x| x.origin.truncate()))
    {
        for tile in sprites.iter() {
            let x = tile.translation.x + 8.0;
            let y = tile.translation.y + 8.0;
            if x >= world_position.x
                && x <= world_position.x + 16.
                && y >= world_position.y
                && y <= world_position.y + 16.
            {
                outline.translation = tile.translation;
                outline.translation.z = 1.;
                return;
            }
        }
        outline.translation.z = -1.;
    }
}
