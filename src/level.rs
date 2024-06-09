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
use bevy::ecs::event::EventReader;
use bevy::window::CursorMoved;

use crate::map::{Edge, HexVertex};
use crate::map::Map;

use super::map;

pub fn create_level(mut command_queue : Commands, asset_server : Res<AssetServer>, mut map : ResMut<map::Map>)
{
    map.spawn(&mut command_queue, asset_server);

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

pub fn mouse_moved(mut cursor_event : EventReader<CursorMoved>, mut window : Query<&mut Window>, mut map : ResMut<map::Map>, 
    camera: Query<(&Camera, &GlobalTransform)>)
{
    for event in cursor_event.read()  
    {
        match camera.single().0.viewport_to_world(camera.single().1, event.position) {
            Some(value) => {
                let vertex_axial : Option<(Vec2, bool)> =
                     map::Map::vertexWorldToAxial(vec3(value.origin.x, 0f32, value.origin.z));
                let hex_axial : Option<Vec2> = map::Map::hexWorldToAxial(vec3(value.origin.x, 0f32, value.origin.z));
                let edge_axial : Option<(Vec2, bool, bool, bool)> = map::Map::edgeWorldToAxial(vec3(value.origin.x, 0f32, value.origin.z));
                match hex_axial {
                    Some(value) => {
                        let hex_axial_rounded : Vec2 = Map::hexAxialRound(value);
                        match map.hexes[(hex_axial_rounded.x + 2f32) as usize][(hex_axial_rounded.y + 2f32) as usize] {
                            Some(valid_hex) => {
                                //println!("Cursor moved to hex with resource : {}, dice number : {}, has robber : {}, axial x : {}, axial y : {}", 
                                    //valid_hex.hex_data.resource as i8, valid_hex.hex_data.dice_num, valid_hex.hex_data.has_robber, hex_axial_rounded.x, hex_axial_rounded.y)
                            },
                            None => ()
                        }
                    },
                    None => ()
                }
                match edge_axial {
                    Some(value) => {
                        let edge : &Option<Edge>;
                        let hex_edge = &map.edges[(value.0.x + 3f32) as usize][(value.0.y + 3f32) as usize];
                        if value.1 == true {
                            edge = &hex_edge.north;
                        }
                        else if value.2 == true {
                            edge = &hex_edge.west;
                        }
                        else {
                            edge = &hex_edge.east;
                        }

                        match edge {
                            Some(valid_edge) => {
                                //println!("Cursor moved to edge q: {}, r: {}, north:{}, west:{}", value.0.x, value.0.y, value.1, value.2);
                            },
                            None => ()
                        }
                    },
                    None => ()
                }
                match vertex_axial {
                    Some(value) =>{
                        let vertex;
                        let hex_vertex = &map.vertices[(value.0.x + 3f32) as usize][(value.0.y + 3f32) as usize];
                        if value.1 == true {
                            vertex = &hex_vertex.bottom;
                        }
                        else {
                            vertex = &hex_vertex.top;
                        }
                        match vertex {
                            Some(valid_vertex) => {
                                println!("Cursor moved to vertex q: {}, r: {}", value.0.x, value.0.y);

                                let vertex_neighbours = map.getVertexNeighbourAxials(value.0, value.1);
                                for i in 0..vertex_neighbours.len() {
                                    //println!("Vertex has neighbour with q: {}, r: {}", 
                                        //vertex_neighbours[i].0.x, vertex_neighbours[i].0.y);
                                }

                                let vertexTouchingHexes = map.getVertexTouchingHexAxials(value.0, value.1);
                                for i in 0 .. vertexTouchingHexes.len(){
                                    //println!("Vertex is touching hex with q: {}, r: {}", 
                                        //vertexTouchingHexes[i].x, vertexTouchingHexes[i].y);
                                }

                                let vertexProtrudingEdges = 
                                    map.getVertexProtrudingEdgeAxials(value.0, value.1);
                                for i in 0.. vertexProtrudingEdges.len() {
                                    println!("Vertex has protruding edge with q: {}, r: {}, isnorth: {}, iswest: {}", 
                                        vertexProtrudingEdges[i].0.x, vertexProtrudingEdges[i].0.y, vertexProtrudingEdges[i].1, vertexProtrudingEdges[i].2);
                                }

                                match valid_vertex.port_data {
                                    Some(port_data) => {
                                        //println!("Cursor moved to vertex with resource! q: {} r: {} Port Resource: {}", value.0.x, value.0.y, port_data.input as i8);
                                    },
                                    None =>()
                                }
                            },
                            None => ()
                        }
                    }
                    None => ()
                }
            }
            None => ()
        }
    }
}