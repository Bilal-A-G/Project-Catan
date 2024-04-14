use bevy::asset::Assets;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::system::{Commands, ResMut};
use bevy::math::{self, vec3, Vec3};
use bevy::pbr::{DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial};
use bevy::prelude::default;
use bevy::render::mesh::{shape, Mesh};
use bevy::transform::components::Transform;
use bevy::render::color::Color;

pub fn spawn_camera(mut command_queue : Commands)
{
    command_queue.spawn(Camera3dBundle{
        transform : Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(vec3(0f32, 0f32, 0f32), vec3(0f32, 1f32, 0f32)),
        ..default()
    });
}

pub fn create_test_scene(mut command_queue : Commands, 
    mut meshes : ResMut<Assets<Mesh>>, 
    mut materials : ResMut<Assets<StandardMaterial>>
)
{
    command_queue.spawn(PbrBundle{
        mesh : meshes.add(Mesh::from(math::primitives::Cuboid{
                half_size : vec3(5f32, 0.1f32, 5f32),
                ..default()
            }
        )),
        material : materials.add(Color::rgb(0.1, 0.5, 0.1)),
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