use bevy::asset::AssetServer;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::system::Commands;
use bevy::math::{vec3, Vec3};
use bevy::pbr::{DirectionalLight, DirectionalLightBundle};
use bevy::prelude::default;
use bevy::render::camera::{OrthographicProjection, Projection};
use bevy::scene::{Scene, SceneBundle};
use bevy::transform::components::Transform;
use bevy::ecs::system::Res;

const MAP_HEIGHT : i8 = 10; 
const MAP_WIDTH : i8 = 10; 

pub fn spawn_camera(mut command_queue : Commands)
{
    command_queue.spawn(Camera3dBundle{
        transform : Transform::from_xyz(0.0, 20.0, 0.0)
            .looking_at(vec3(0f32, 0f32, 0f32), vec3(0f32, 1f32, 0f32)),
        projection : Projection::Orthographic(OrthographicProjection{
            scale : 0.03f32,
            ..default()
        }),
        ..default()
    });
}

pub fn spawn_lights(mut command_queue : Commands,
    asset_server : Res<AssetServer>
)
{
    let hexagon : bevy::prelude::Handle<Scene> = asset_server.load("Hex.glb#Scene0");
    let hex_radius : f32 = 2.1f32;
    let initial_translation : Vec3 = Transform::from_xyz(-4.0f32, 0.0f32, -4.5f32).translation;

    for i in 0..MAP_HEIGHT/2
    {
        let offset_z : f32 = initial_translation.z + if i % 2 == 0 {hex_radius - 1.0f32} else {0.0f32};
        let offset_x : f32 = initial_translation.x + (hex_radius - 0.2f32) * i as f32;
        for j in 0..MAP_WIDTH/2
        {
            command_queue.spawn(SceneBundle{
                scene : hexagon.clone(),
                transform : Transform::from_xyz(offset_x, initial_translation.y, offset_z + j as f32 * (hex_radius + 0.05f32)),
                ..default()
            });
        }
    }

    command_queue.spawn(DirectionalLightBundle{
        directional_light : DirectionalLight{
            ..default()
        },
        transform : Transform::from_xyz(0f32, 10f32, 0f32).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}