use std::borrow::BorrowMut;
use std::rc::Rc;

use bevy::asset::{AssetServer, Assets, Handle};
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::system::{Commands, ResMut};
use bevy::math::primitives::Cuboid;
use bevy::math::{vec3, Vec3};
use bevy::pbr::{DirectionalLight, DirectionalLightBundle, PbrBundle};
use bevy::prelude::default;
use bevy::render::camera::{OrthographicProjection, Projection, ScalingMode};
use bevy::render::mesh::Mesh;
use bevy::scene::{Scene, SceneBundle};
use bevy::transform::components::Transform;
use bevy::ecs::system::Res;

#[derive(Default)]
pub struct Edge
{
    vertex_from : Vec3,
    vertex_to : Vec3,
    neighbours : Vec<i32>
}

const MAP_HEIGHT : i8 = 2; 
const MAP_WIDTH : i8 = 10; 

pub fn spawn_camera(mut command_queue : Commands)
{
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
}

pub fn spawn_lights(mut command_queue : Commands,
    asset_server : Res<AssetServer>,
    mut meshes : ResMut<Assets<Mesh>>
)
{
    let hexagon : bevy::prelude::Handle<Scene> = asset_server.load("Hex.glb#Scene0");
    let hex_radius : f32 = 2.0f32;
    let initial_translation : Vec3 = Transform::from_xyz(-4.0f32, 0.0f32, -2.0f32).translation;

    let cuboid : Handle<Mesh> = meshes.add(Cuboid{
        half_size : vec3(0.1f32, 0.1f32, 0.1f32), 
        ..default()
    });

    let mut edges : Vec<Edge> = Vec::new();

    for i in 0..MAP_HEIGHT/2
    {
        let offset_z : f32 = initial_translation.z + if i % 2 == 0 {hex_radius - 1.0f32} else {0.0f32};
        let offset_x : f32 = initial_translation.x + (hex_radius - 0.2f32) * i as f32;
        for j in 0..MAP_WIDTH/2
        {
            command_queue.spawn(SceneBundle{
                scene : hexagon.clone(),
                transform : Transform::from_xyz(offset_x, initial_translation.y,  offset_z - j as f32 * (hex_radius + 0.15f32)),
                ..default()
            });
            
            let center : Vec3 = vec3(offset_x, initial_translation.y + 10.0f32, -j as f32 * hex_radius);

            let bottom : Vec3 = center + vec3(hex_radius/1.9f32, 0.0f32, 0.0f32);
            let top : Vec3 = center - vec3(hex_radius/1.9f32, 0.0f32, 0.0f32);
            let top_right : Vec3 = center - vec3(hex_radius * 0.3f32, 0.0f32, hex_radius/2.0f32);
            let top_left : Vec3 = center - vec3(hex_radius * 0.3f32, 0.0f32, -hex_radius/2.0f32);
            let bottom_right : Vec3 = center - vec3(-hex_radius * 0.3f32, 0.0f32, hex_radius/2.0f32);
            let bottom_left : Vec3 = center + vec3(hex_radius * 0.3f32, 0.0f32, hex_radius/2.0f32);
            
            let mut top_left_edge  = Edge{vertex_from : top_left, vertex_to : top, ..default()};
            let mut top_right_edge= Edge{vertex_from : top, vertex_to : top_right, ..default()};
            let mut right_edge = Edge{vertex_from : top_right, vertex_to : bottom_right, ..default()};
            let mut bottom_right_edge =Edge{vertex_from : bottom_right, vertex_to : bottom, ..default()};
            let mut bottom_left_edge = Edge{vertex_from : bottom, vertex_to : bottom_left, ..default()};
            let mut left_edge= Edge{vertex_from : bottom_left, vertex_to : top_left, ..default()};

            if j == 0
            {
                top_left_edge.neighbours.extend([1, 5]);
                top_right_edge.neighbours.extend([0, 2]);
                right_edge.neighbours.extend([1, 3]);
                bottom_right_edge.neighbours.extend([2, 4]);
                bottom_left_edge.neighbours.extend([3, 5]);
                left_edge.neighbours.extend([4, 0]);

                edges.extend([top_left_edge, top_right_edge, right_edge, bottom_right_edge, bottom_left_edge, left_edge]);
            }
            else 
            {
                let previous_top_right : i32;
                let previous_right : i32;
                let previous_bottom_right : i32;
                let edges_size: i32 = edges.len() as i32;

                if j == 1
                {
                    previous_top_right = 1;
                    previous_right = 2;
                    previous_bottom_right = 3;
                }
                else 
                {
                    previous_top_right = edges_size - 4;
                    previous_right = edges_size - 3;
                    previous_bottom_right = edges_size - 2;
                }

                top_left_edge.neighbours.extend([previous_top_right, previous_right, edges_size + 1]);
                top_right_edge.neighbours.extend([edges_size + 0, edges_size + 2]);
                right_edge.neighbours.extend([edges_size + 1, edges_size + 3]);
                bottom_right_edge.neighbours.extend([edges_size + 2, edges_size + 4]);
                bottom_left_edge.neighbours.extend([edges_size + 3, previous_bottom_right, previous_right]);

                edges[previous_top_right as usize].neighbours.extend([edges_size + 0]);
                edges[previous_right as usize].neighbours.extend([edges_size + 0, edges_size + 4]);
                edges[previous_bottom_right as usize].neighbours.extend([edges_size + 4]);

                edges.extend([top_left_edge, top_right_edge, right_edge, bottom_right_edge, bottom_left_edge]);
            }
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