use bevy::asset::{AssetServer, Assets};
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::{Commands, Query, ResMut, Resource};
use bevy::input::mouse::MouseMotion;
use bevy::math::{vec3, Vec3};
use bevy::pbr::{DirectionalLight, DirectionalLightBundle};
use bevy::prelude::default;
use bevy::render::camera::{OrthographicProjection, Projection, ScalingMode};
use bevy::render::mesh::Mesh;
use bevy::scene::{Scene, SceneBundle};
use bevy::transform::components::Transform;
use bevy::ecs::system::Res;
use bevy::window::{CursorMoved, Window};

#[derive(Copy, Clone)]
pub struct Vertex 
{
    has_settlement : bool,
    is_hex_center : bool,
    world_coordinates : Vec3
}

const MAP_HEIGHT : i8 = 10; 
const MAP_WIDTH : i8 = 10; 

#[derive(Resource)]
pub struct Map
{
    vertices : Option<Vec<Vec<Vec<Option<Vertex>>>>>
}

impl Map 
{
    pub fn create_new() -> Map
    {
        Map { vertices: None }
    }

    pub fn spawn(&mut self, command_queue : &mut Commands, asset_server : Res<AssetServer>)
    {
        let hexagon : bevy::prelude::Handle<Scene> = asset_server.load("Hex.glb#Scene0");

        let hex_radius : f32 = 2.0f32;
        let initial_translation : Vec3 = Transform::from_xyz(-4.0f32, 0.0f32, 7.0f32).translation;
    
        let max_x : i8 = MAP_WIDTH/2 + f32::floor((MAP_HEIGHT as f32/2f32) / 2f32) as i8 + 1;
        let max_z : i8 = max_x + 1;
        let max_y : i8 = MAP_HEIGHT/2 + 1;

        self.vertices = Some
        (vec![
            vec![
                vec![None; max_z as usize]; 
                max_y as usize
            ]; 
            max_x as usize
        ]);

        for j in 0..MAP_HEIGHT/2
        {
            let offset_z : f32 = initial_translation.z + if j % 2 == 0 {hex_radius - 0.90f32} else {0.0f32};
            let offset_x : f32 = initial_translation.x + (hex_radius) * j as f32;
    
            for i in 0..MAP_WIDTH/2
            {
                command_queue.spawn(SceneBundle{
                    scene : hexagon.clone(),
                    transform : Transform::from_xyz(offset_x, initial_translation.y,  offset_z - i as f32 * (hex_radius + 0.22f32)),
                    ..default()
                });
                
                let center : Vec3 = vec3(offset_x, initial_translation.y,  offset_z - i as f32 * (hex_radius + 0.16f32));
    
                let bottom : Vec3 = center + vec3(hex_radius/1.9f32, 0.0f32, 0.0f32);
                let top : Vec3 = center - vec3(hex_radius/1.9f32, 0.0f32, 0.0f32);
                let top_right : Vec3 = center - vec3(hex_radius * 0.3f32, 0.0f32, hex_radius/2.0f32);
                let top_left : Vec3 = center - vec3(hex_radius * 0.3f32, 0.0f32, -hex_radius/2.0f32);
                let bottom_right : Vec3 = center - vec3(-hex_radius * 0.3f32, 0.0f32, hex_radius/2.0f32);
                let bottom_left : Vec3 = center + vec3(hex_radius * 0.3f32, 0.0f32, hex_radius/2.0f32);
    
                let index_to_grid : Vec3 = index_to_grid(i, j);
                let x : usize = index_to_grid.x as usize;
                let y : usize = index_to_grid.y as usize;
                let z : usize = index_to_grid.z as usize;
                
                match self.vertices{
                    Some(ref mut vec) => {

                        vec[x][y][z] = Some(Vertex{has_settlement: false, world_coordinates: center, is_hex_center: true});
                                
                        match vec[x][y + 1][z] {
                            Some(_) => (),
                            None => {
                                vec[x][y + 1][z] = Some(Vertex{has_settlement: false, world_coordinates: bottom, is_hex_center: false});
                            }
                        };
                        match vec[x - 1][y][z + 1] {
                            Some(_) => (),
                            None => {
                                vec[x - 1][y][z + 1] = Some(Vertex{has_settlement: false, world_coordinates: top, is_hex_center: false});
                            }
                        };
                        match vec[x][y][z + 1] {
                            Some(_) => (),
                            None => {
                                vec[x][y][z + 1] = Some(Vertex{has_settlement: false, world_coordinates: top_right, is_hex_center: false});
                            }
                        };
                        match vec[x - 1][y][z] {
                            Some(_) => (),
                            None => {
                                vec[x - 1][y][z] = Some(Vertex{has_settlement: false, world_coordinates: top_left, is_hex_center: false});
                            }
                        };
                        match vec[x][y + 1][z + 1] {
                            Some(_) => (),
                            None => {
                                vec[x][y + 1][z + 1] = Some(Vertex{has_settlement: false, world_coordinates: bottom_right, is_hex_center: false});
                            }
                        };
                        match vec[x - 1][y + 1][z] {
                            Some(_) => (),
                            None => {
                                vec[x - 1][y + 1][z] = Some(Vertex{has_settlement: true, world_coordinates: bottom_left, is_hex_center: false});
                            }
                        };
                    },
                    None => (),
                }
            }
        }
    }

    pub fn print_vertices(&self)
    {
        match self.vertices {
            Some(ref vec) => {
                for i in 0..vec.len() {
                    for j in 0..vec[i].len()  {
                        for k in 0..vec[i][j].len()  {
                            match vec[i][j][k] {
                                Some(value) => {
                                    println!("grid coords = ({}, {}, {}), world coords = ({}, {}, {}), has settlement = ({}), is hex center = ({})",
                                    i, j, k, 
                                    value.world_coordinates.x, value.world_coordinates.y, value.world_coordinates.z, 
                                    value.has_settlement, 
                                    value.is_hex_center);
                                },
                                None => ()
                            }
                        }
                    }
                }
            },
            None => return
        }
    }

    pub fn test(&self)
    {
        match &self.vertices {
            Some(vec) => {
                match vec[1][0][5] {
                    Some(value) => {
                        println!("{}", value.is_hex_center);
                        println!("Top left vertex = (x:{}, z:{})", value.world_coordinates.x, value.world_coordinates.z);
                    },
                    None =>()
                }
            },
            None => return
        };
    }
}

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

pub fn index_to_grid(i : i8, j : i8) -> Vec3
{
    let grid_coords : Vec3 = Vec3::new(
        i as f32 + f32::floor(j as f32 / 2f32) + (j as f32 % 2f32) + 1f32, 
        j as f32, 
        f32::floor(j as f32 / 2f32) - i as f32 + MAP_WIDTH as f32/2f32
    );

    return grid_coords;
}

pub fn mouse_moved(mut cursor_event : EventReader<CursorMoved>, mut window : Query<&mut Window>, mut map : ResMut<Map>)
{
    for event in cursor_event.read()
    {
        map.test();
        println!("Cursor moved! x-{} y-{}", event.position.x - window.single().resolution.width()/2f32, event.position.y - window.single().resolution.height()/2f32)
    }
}

pub fn initialize_map(mut command_queue : Commands, asset_server : Res<AssetServer>, mut map : ResMut<Map>)
{
    map.spawn(&mut command_queue, asset_server);
    map.print_vertices();
}

pub fn spawn_lights(mut command_queue : Commands,
    asset_server : Res<AssetServer>,
    mut meshes : ResMut<Assets<Mesh>>
)
{
    command_queue.spawn(DirectionalLightBundle{
        directional_light : DirectionalLight{
            ..default()
        },
        transform : Transform::from_xyz(0f32, 10f32, 0f32).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}