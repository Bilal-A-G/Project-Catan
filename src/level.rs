use bevy::asset::AssetServer;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::system::{Commands, Query, ResMut};
use bevy::math::{vec2, vec3, Vec2, Vec3};
use bevy::pbr::{DirectionalLight, DirectionalLightBundle};
use bevy::prelude::default;
use bevy::render::camera::{OrthographicProjection, Projection, ScalingMode};
use bevy::transform::components::Transform;
use bevy::transform::components::GlobalTransform;
use bevy::ecs::system::Res;
use bevy::window::Window;
use bevy::render::camera::Camera;

use super::map;

pub fn place_settlement(window : Query<&Window>, mut map : ResMut<map::Map>, camera: Query<(&Camera, &GlobalTransform)>)
{
    let mut cursor_position : Vec2 = vec2(0f32, 0f32);

    match window.single().cursor_position() {
        Some(position) => {
            cursor_position = position;
        },
        None => {return;}
    }

    match camera.single().0.viewport_to_world(camera.single().1, cursor_position) {
        Some(value) => {
            let index_position : Option<Vec3> = map.get_vertex_at_position(value.origin.x, value.origin.z, false);
            match index_position {
                Some(position) => {
                    println!("Cliked at! x: {} y: {} z: {}", position.x, position.y, position.z);
                    match map.try_place_settlement(position){
                        true => {println!("Placed settlement!");},
                        false => {println!("Failed to place settlement!");}
                    }
                },
                None => ()
            }
        },
        None =>()
    }
}

pub fn create_level(mut command_queue : Commands, asset_server : Res<AssetServer>, mut map : ResMut<map::Map>)
{
    map.spawn(&mut command_queue, asset_server);
    map.print_vertices();

    command_queue.spawn(Camera3dBundle{
        transform : Transform::from_xyz(0.0, 20.0, 0.0)
            .looking_at(vec3(0f32, 0f32, 0f32), vec3(0f32, 1f32, 0f32)),
        projection : Projection::Orthographic(OrthographicProjection{
            scale : 0.03f32,
            scaling_mode : ScalingMode::AutoMax { max_width: 800.0f32, max_height:  800.0f32},
            ..default()
        }),
        ..default()
    });

    command_queue.spawn(DirectionalLightBundle{
        directional_light : DirectionalLight{
            ..default()
        },
        transform : Transform::from_xyz(0f32, 10f32, 0f32).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}