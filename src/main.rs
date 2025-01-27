use animation_graph::{CharacterAnimationGraph, Variable};
use bevy::{prelude::*, render::camera::ScalingMode};
use camera::{CameraSettings, EditorCamera, GameCameraPlugin};
use world::WorldPlugin;

mod animation_graph;
mod camera;
mod error;
mod prelude;
mod world;

#[derive(Component)]
pub struct GamePlayer;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::Mailbox,
                        name: Some("Fishing is Boring".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(WorldPlugin)
        .add_plugins(GameCameraPlugin)
        .add_systems(Startup, create_player)
        .add_systems(Update, update_player)
        .add_systems(FixedUpdate, move_player)
        .run();
}

pub fn update_player(
    keys: Res<ButtonInput<KeyCode>>,
    query: Single<(&GamePlayer, &mut CharacterAnimationGraph)>,
    camera_query: Single<&Camera, With<EditorCamera>>,
) {
    if camera_query.into_inner().is_active {
        return;
    }
    let (_, mut graph) = query.into_inner();
    if keys.just_pressed(KeyCode::Space) {
        info!("SPACE");
        graph.set_variable("attacking", Variable::Bool(true));
    }
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    query: Single<(
        &GamePlayer,
        &mut Transform,
        &mut Sprite,
        &mut CharacterAnimationGraph,
    )>,
    camera_query: Single<&Camera, With<EditorCamera>>,
) {
    if camera_query.into_inner().is_active {
        return;
    }

    let (_, mut player_transform, mut sprite, mut graph) = query.into_inner();

    let delta = time.delta_secs();

    let speed = 200.0;

    let mut movement_vector = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        graph.set_variable("directionY", Variable::Enum("up".to_string()));

        movement_vector.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        graph.set_variable("directionY", Variable::Enum("down".to_string()));
        movement_vector.y -= 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        graph.set_variable("directionX", Variable::Enum("left".to_string()));
        movement_vector.x += 1.0;
    } else if keys.pressed(KeyCode::KeyD) {
        graph.set_variable("directionX", Variable::Enum("right".to_string()));
        movement_vector.x -= 1.0;
    }

    if (keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::KeyS))
        && !keys.pressed(KeyCode::KeyD)
        && !keys.pressed(KeyCode::KeyA)
    {
        graph.set_variable("directionX", Variable::Enum("none".to_string()));
    }

    graph.set_variable("walking", Variable::Bool(movement_vector != Vec3::ZERO));

    if let Some(next_frame) = graph.get_next_index(delta) {
        sprite.texture_atlas.as_mut().unwrap().index = next_frame;
        if let Some(animation) = graph.get_current_animation() {
            sprite.flip_x = animation.flip_x;
        }
    }

    if movement_vector == Vec3::ZERO {
        return;
    }

    player_transform.translation += movement_vector.normalize() * speed * delta;
}

pub fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    camera_settings: Res<CameraSettings>,
) {
    let image_handle = asset_server.load("player.png");
    let atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 10, None, None);
    let layout = texture_atlas_layouts.add(atlas_layout);

    commands
        .spawn((
            GamePlayer,
            Sprite {
                flip_x: true,
                ..Sprite::from_atlas_image(
                    image_handle.clone(),
                    TextureAtlas {
                        layout: layout.clone(),
                        index: 0,
                    },
                )
            },
            CharacterAnimationGraph::new(),
            Transform::from_xyz(0., 0., 2.),
        ))
        .with_child((
            Camera2d,
            Camera {
                order: 1,
                ..Camera::default()
            },
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
            Transform::from_xyz(0.0, 0.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
}
