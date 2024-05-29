use bevy::math::{vec2, vec3, Vec2, Vec3};
use bevy::ecs::system::{Resource, Res, Commands};
use bevy::transform::components::Transform;
use bevy::scene::{Scene, SceneBundle};
use bevy::asset::{self, AssetServer};
use bevy::utils::default;

use crate::common::common::PortData;
use crate::common::common::ResourceType;

const MAP_SIZE : i8 = 2; 

const HEX_RADIUS : f32 = 1.0f32;
const DEFAULT_HEX_SIZE : f32 = 0.2f32;
const INITIAL_TRANSLATION : Vec3 = Transform::from_xyz(0.0f32, 0.0f32, 0.0f32).translation;

const HEX_HEIGHT : f32 = (HEX_RADIUS + DEFAULT_HEX_SIZE) * 2f32;

const HORIZONTAL_DISTANCE : f32 = 1.732f32 * (HEX_RADIUS + DEFAULT_HEX_SIZE);
const VERTICAL_DISTANCE : f32 = 3f32/4f32 * HEX_HEIGHT;

const CLOSENESS_THRESHOLD : f32 = 0.3f32;

#[derive(Clone)]
pub struct Vertex 
{
    world_coordinates : Vec3,
    port_data : Option<PortData>
}

pub struct PortPosition
{
    axial_coordinates : Vec2,
    is_bottom : bool
}

#[derive(Copy, Clone)]
pub struct Edge 
{
    world_coordinates : Vec3
}

#[derive(Clone)]
pub struct HexVertex 
{
    top : Option<Vertex>,
    bottom : Option<Vertex>
}

#[derive(Copy, Clone)]
pub struct HexEdge
{
    north : Option<Edge>,
    west : Option<Edge>,
    east : Option<Edge>
}

#[derive(Copy, Clone)]
pub struct Hex
{
    center_coordinates : Vec3
}

#[derive(Resource)]
pub struct Map
{
    hexes : Option<Vec<Vec<Option<Hex>>>>,
    vertices : Option<Vec<Vec<HexVertex>>>,
    edges : Option<Vec<Vec<HexEdge>>>,

    ports : Vec<PortData>,
    port_vertices : Vec<PortPosition>
}

impl Map 
{
    pub fn create_new() -> Map
    {
        Map 
        { 
            hexes: Some(vec![
                vec![None; (MAP_SIZE * 2 + 1) as usize]; 
                (MAP_SIZE * 2 + 1) as usize
            ]),
            vertices: Some(vec![
                vec![HexVertex{top : None, bottom : None}; ((MAP_SIZE * 2 + 1) + 2) as usize];
                ((MAP_SIZE * 2 + 1) + 2) as usize               
            ]),
            edges: Some(vec![
                vec![HexEdge{north : None, west : None, east : None}; ((MAP_SIZE * 2 + 1) + 2) as usize];
                ((MAP_SIZE * 2 + 1) + 2) as usize
            ]),
            ports: vec![
                PortData{input: ResourceType::Anything, num_inputs : 3}, 
                PortData{input: ResourceType::Anything, num_inputs : 3},
                PortData{input: ResourceType::Anything, num_inputs : 3}, 
                PortData{input: ResourceType::Anything, num_inputs : 3},

                PortData{input: ResourceType::Wood, num_inputs : 2}, 
                PortData{input: ResourceType::Sheep, num_inputs : 2},
                PortData{input: ResourceType::Brick, num_inputs : 2},
                PortData{input: ResourceType::Wheat, num_inputs : 2},
                PortData{input: ResourceType::Stone, num_inputs : 2}
                ],
            port_vertices: vec![
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},
                
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(0f32, 0f32), is_bottom: false}
            ]
        }
    }

    pub fn hexAxialRound(hex_frac_axial : Vec2) -> Vec2 
    {
        let rounded_q : f32 = f32::round(hex_frac_axial.x);
        let rounded_r : f32 = f32::round(hex_frac_axial.y);

        let remainder_q : f32 = hex_frac_axial.x - rounded_q;
        let remainder_r : f32 = hex_frac_axial.y - rounded_r;

        let mut rounded_axial : Vec2 = vec2(0f32, 0f32);

        if f32::abs(remainder_q) > f32::abs(remainder_r)
        {
            rounded_axial.x = rounded_q + f32::round(remainder_q + 0.5f32 * remainder_r);
            rounded_axial.y = rounded_r;
        }
        else {
            rounded_axial.x = rounded_q;
            rounded_axial.y = rounded_r + f32::round(remainder_r + 0.5f32 * remainder_q);
        }

        return rounded_axial;
    }

    pub fn hexWorldToAxial(world : Vec3) -> Vec2
    {
        let y_axial : f32 = (world.x - INITIAL_TRANSLATION.x) / VERTICAL_DISTANCE;
        let x_axial : f32 = (world.z - INITIAL_TRANSLATION.z - y_axial * (HORIZONTAL_DISTANCE / 2f32)) / HORIZONTAL_DISTANCE;
        return vec2(x_axial, y_axial);
    }

    pub fn cubeToDist(cube : Vec3) -> f32
    {
        return (f32::abs(cube.x) + f32::abs(cube.y) + f32::abs(cube.z))/2f32;
    }

    pub fn vertexWorldToAxial(world : Vec3) -> Option<(Vec2, bool)>
    {
        let hex_frac_axial : Vec2 = Self::hexWorldToAxial(world);
        let rounded_hex_axial : Vec2 = Self::hexAxialRound(hex_frac_axial);
        let hex_cube_coords : Vec3 = Self::hexToCube(rounded_hex_axial.x as i8, rounded_hex_axial.y as i8);
        let distance_from_center : f32 = Self::cubeToDist(hex_cube_coords);

        if distance_from_center > MAP_SIZE as f32 {
            return None;
        }

        let center : Vec3 = Self::hexAxialToWorld(rounded_hex_axial.x as i8, rounded_hex_axial.y as i8);
        let mut vertex_i : i8 = -1;
        for i in 0i8..6i8 {
            if (Self::getCorners(center, i) - world).length() <= CLOSENESS_THRESHOLD {
                vertex_i = i;
                break;
            }
        }
        if vertex_i == -1 {
            return None;
        } 
        let q_offset : i8 = Self::vertexQOffsetFromI(vertex_i);
        let r_offset : i8 = Self::vertexROffsetFromI(vertex_i);
        return Some((vec2(q_offset as f32, r_offset as f32) + rounded_hex_axial, vertex_i % 2 == 0));
    }

    pub fn hexToCube(q : i8, r : i8) -> Vec3
    {
        return vec3((r + q) as f32, -r as f32, q as f32);
    }

    pub fn edgeWorldToAxial(world : Vec3) -> Option<(Vec2, bool, bool, bool)>
    {
        let mut hex_frac_axial : Vec2 = Self::hexWorldToAxial(world);
        hex_frac_axial = vec2(f32::clamp(hex_frac_axial.x, -MAP_SIZE as f32, MAP_SIZE as f32), 
            f32::clamp(hex_frac_axial.y, -MAP_SIZE as f32, MAP_SIZE as f32));
        let rounded_hex_axial : Vec2 = Self::hexAxialRound(hex_frac_axial);
        let center : Vec3 = Self::hexAxialToWorld(rounded_hex_axial.x as i8, rounded_hex_axial.y as i8);

        let mut edge_i : i8 = -1;
        for i in 0i8..6i8 {
            if (Self::getEdges(center, i) - world).length() <= CLOSENESS_THRESHOLD {
                edge_i = i;
                break;
            }
        }
        if edge_i == -1 {
            return None;
        }
        let q_offset : i8 = Self::edgeQOffsetFromI(edge_i);
        let r_offset : i8 = Self::edgeROffsetFromI(edge_i);

        //is north, is west and is east booleans in tuple
        return Some((vec2(q_offset as f32, r_offset as f32) + rounded_hex_axial, 
            edge_i == 0 || edge_i == 3, 
            edge_i == 2 || edge_i == 5, 
            edge_i == 1 || edge_i == 4));
    }

    pub fn hexAxialToWorld(q_offset : i8, r_offset : i8) -> Vec3 
    {
        let x_position : f32 = INITIAL_TRANSLATION.x + r_offset as f32 * VERTICAL_DISTANCE;
        let z_position : f32 = (INITIAL_TRANSLATION.z + q_offset as f32 * HORIZONTAL_DISTANCE) + 
            r_offset as f32 * (HORIZONTAL_DISTANCE / 2f32);
        
        return vec3(x_position, INITIAL_TRANSLATION.y, z_position);
    }

    pub fn vertexAxialToWorld(q_offset : i8, r_offset : i8, center : Vec3, is_bottom : bool ) -> Vec3 
    {
        let hex_axial : Vec2 = Self::hexWorldToAxial(center);
        let vertex_q_offset : i8 = q_offset - hex_axial.x as i8;
        let vertex_r_offset : i8 = r_offset - hex_axial.y as i8;

        let world_position : Vec3 = Self::getCorners(center, 
            Self::vertexIFromOffset(vertex_q_offset, vertex_r_offset, is_bottom));
        
        return world_position;
    }

    pub fn edgeAxialToWorld(q_offset : i8, r_offset : i8, center : Vec3, is_north : bool, is_west : bool, is_east : bool) -> Vec3
    {
        let hex_axial : Vec2 = Self::hexWorldToAxial(center);
        let edge_q_offset : i8 = q_offset - hex_axial.x as i8;
        let edge_r_offset : i8 = r_offset - hex_axial.y as i8;

        let world_position : Vec3 = Self::getEdges(center, 
            Self::edgeIFromOffset(edge_q_offset, edge_r_offset, is_north, is_west, is_east));

        return world_position;
    }

    pub fn vertexQOffsetFromI(i : i8) -> i8
    {
        if i == 0 || i == 1 || i == 3 || i == 4 {
            return 0;
        }

        if i == 2{
            return 1;
        }
        else {
            return -1;
        }
    }

    pub fn vertexROffsetFromI(i : i8) -> i8
    {
        if i == 0 || i == 3 {
            return 0;
        }

        if i % 2 == 0{
            return -1;
        }
        else {
            return 1;
        }
    }

    pub fn edgeQOffsetFromI(i : i8) -> i8 
    {
        if i == 0 || i == 2 || i == 3 || i == 4{
            return 0;
        }
        else if i == 1{
            return -1;
        }
        else {
            return 1;
        }
    }

    pub fn edgeROffsetFromI(i : i8) -> i8 
    {
        if i == 1 || i == 2 || i == 3 || i == 4{
            return 0;
        }
        else {
            return -1;
        }
    }

    pub fn vertexIFromOffset(q_offset : i8, r_offset : i8, isbottom : bool) -> i8 
    {
        if q_offset == 0 && r_offset == 0 {
            if isbottom {return  0;}
            else {return 3;}
        }
        else if q_offset == 0 && r_offset != 0 {
            if isbottom {return 2;}
            else {return 5;}
        }
        else if q_offset == -1 && r_offset == 1 {
            return 1;
        }
        else if q_offset == 1 && r_offset == -1 {
            return 4;
        }
        else{
            print!("Vertex i value cannot be found!");
            return -1;
        }
    }

    pub fn edgeIFromOffset(q_offset : i8, r_offset : i8, is_north : bool, is_west : bool, is_east : bool) -> i8
    {
        if q_offset == 0 && r_offset == 0 {
            if is_west {
                return 2;
            }
            else if is_north {
                return 3;
            }
            else if is_east {
                return  4;
            }
        }

        if q_offset == 0 && r_offset == -1 {
            return 0;
        }
        else if q_offset == -1 && r_offset == 0 {
            return 1;
        }
        else if q_offset == 1 && r_offset == -1 {
            return 5;
        }
        else {
            print!("Edge i value cannot be found!");
            return -1;
        }
    }

    pub fn getCorners(center : Vec3, i : i8) -> Vec3
    {
        let degree_angle : f32 = 30f32 + (60f32 * i as f32 - 30f32);
        let rad_angle : f32 = 3.14f32 / 180f32 * degree_angle;
        let vertex_coord : Vec3 = vec3(
            center.x + (HEX_RADIUS + DEFAULT_HEX_SIZE) * f32::cos(rad_angle), 
            center.y,
            center.z + (HEX_RADIUS + DEFAULT_HEX_SIZE) * f32::sin(rad_angle));
        
        return vertex_coord;
    }

    pub fn getEdges(center : Vec3, i : i8) -> Vec3
    {
        let degree_angle : f32 = 60f32 * i as f32 - 30f32;
        let rad_angle : f32 = 3.14f32 / 180f32 * degree_angle;
        let edge_coord : Vec3 = vec3(
            center.x + (HEX_RADIUS + DEFAULT_HEX_SIZE) * f32::cos(rad_angle), 
            center.y,
            center.z + (HEX_RADIUS + DEFAULT_HEX_SIZE) * f32::sin(rad_angle));
        
        return edge_coord;
    }

    //For manhattan distance, use as is
    pub fn vertexToCube(i : i8, hex_q : i8, hex_r : i8) -> Vec3 
    {   
        let hex_cube : Vec3 = vec3((hex_r + hex_q) as f32, -hex_r as f32, hex_q as f32);
        let mut vertex_cube_offset : Vec3 = vec3(0f32, 0f32, 0f32);

        match i {
            0 => {vertex_cube_offset.y -= 1f32}
            1 => {vertex_cube_offset.x -= 1f32; vertex_cube_offset.y -= 1f32}
            2 => {vertex_cube_offset.x -= 1f32}
            3 => {vertex_cube_offset.x -= 1f32; vertex_cube_offset.z += 1f32}
            4 => {vertex_cube_offset.z += 1f32}
            5 => {vertex_cube_offset.z += 1f32; vertex_cube_offset.y -= 1f32}
            _ => ()
        }

        return hex_cube + vertex_cube_offset;
    }

    //For manhattan distance, add 1
    pub fn edgeToCube(i : i8, hex_q : i8, hex_r : i8) -> Vec3 
    {
        let hex_cube : Vec3 = vec3((hex_r + hex_q) as f32, -hex_r as f32, hex_q as f32);
        let mut edge_cube_offset : Vec3 = vec3(0f32, 0f32, 0f32);

        match i {
            0 => {edge_cube_offset.y -= 1f32; edge_cube_offset.x -= 0.5f32}
            1 => {edge_cube_offset.x -= 1f32; edge_cube_offset.y -= 0.5f32}
            2 => {edge_cube_offset.x -= 1f32; edge_cube_offset.z += 0.5f32}
            3 => {edge_cube_offset.z += 1f32; edge_cube_offset.x -= 0.5f32}
            4 => {edge_cube_offset.z += 1f32; edge_cube_offset.y -= 0.5f32}
            5 => {edge_cube_offset.y -= 1f32; edge_cube_offset.z += 0.5f32}
            _ => ()
        }

        return hex_cube + edge_cube_offset;
    }

    pub fn spawn(&mut self, command_queue : &mut Commands, asset_server : Res<AssetServer>) 
    {
        let hexagon : bevy::prelude::Handle<Scene> = asset_server.load("Hex.glb#Scene0");
        
        for q_offset in -MAP_SIZE .. MAP_SIZE + 1  
        {
            let mut r_lower_bounds : i8 = -MAP_SIZE - q_offset;
            let mut r_upper_bounds : i8 = MAP_SIZE - q_offset;

            r_lower_bounds = i8::clamp(r_lower_bounds, -MAP_SIZE, MAP_SIZE);
            r_upper_bounds = i8::clamp(r_upper_bounds, -MAP_SIZE, MAP_SIZE);

            for r_offset in r_lower_bounds .. r_upper_bounds + 1
            {
                let world_position : Vec3 = Self::hexAxialToWorld(q_offset, r_offset);

                command_queue.spawn(SceneBundle{
                    scene : hexagon.clone(),
                    transform : Transform::from_translation(world_position).
                        with_scale(vec3(HEX_RADIUS, HEX_RADIUS, HEX_RADIUS)),
                    ..default()
                });

                let x_index : usize = (q_offset + MAP_SIZE) as usize;
                let y_index : usize = (r_offset + MAP_SIZE) as usize;

                println!("\n Coords of hex = x:{}, y:{}", q_offset, r_offset);

                for i in 0i8..6i8 {
                    let corner_vertex : Vec3 = Self::getCorners(world_position, i);
                    let q_vertex_offset : i8 = q_offset + Self::vertexQOffsetFromI(i);
                    let r_vertex_index : i8 = r_offset + Self::vertexROffsetFromI(i);
                    let is_bottom : bool = i % 2 == 0;

                    let x_vertex_index : usize = (q_vertex_offset + MAP_SIZE + 1) as usize; 
                    let y_vertex_index : usize = (r_vertex_index + MAP_SIZE + 1) as usize;

                    //let calculated_world_pos : Vec3 = Self::vertexAxialToWorld(q_vertex_offset, r_vertex_index, world_position, is_bottom);
                    //println!("World X = {}, World Y = {}, Calc World X = {}, Calc World Y = {}", corner_vertex.x, corner_vertex.z, calculated_world_pos.x, calculated_world_pos.z);

                    match self.vertices {
                        Some(ref mut vertices) => {
                            if is_bottom {
                                vertices[x_vertex_index][y_vertex_index].bottom = 
                                    Some(Vertex{world_coordinates : corner_vertex, port_data : None});
                            }
                            else {
                                vertices[x_vertex_index][y_vertex_index].top = 
                                    Some(Vertex{world_coordinates : corner_vertex, port_data : None});
                            }
                        }
                        None => ()
                    }
                }

                for i in 0i8 .. 6i8  {
                    let border_edge : Vec3 = Self::getEdges(world_position, i);
                    let q_edge_offset :  i8 = q_offset + Self::edgeQOffsetFromI(i);
                    let r_edge_offset : i8 = r_offset + Self::edgeROffsetFromI(i);

                    let is_north : bool = i == 0 || i == 3;
                    let is_west : bool = i == 2 || i == 5;
                    let is_east : bool = i == 1 || i == 4;

                    let x_edge_index : usize = (q_edge_offset + MAP_SIZE + 1) as usize; 
                    let y_edge_index : usize = (r_edge_offset + MAP_SIZE + 1) as usize;

                    //let cube_coords : Vec3 = Self::edgeToCube(i, q_offset, r_offset);
                    //println!("X = {}, Y = {}, Z = {}", cube_coords.x, cube_coords.y, cube_coords.z);

                    //println!("Edge Q = {}, Edge R = {}, IsNorth = {}, IsWest = {}, IsEast = {}", 
                        //q_edge_offset, r_edge_offset, is_north, is_west, is_east);

                    //let calculated_offset : Vec2 = Self::edgeWorldToAxial(border_edge).0;
                    //println!("X = {}, Y = {}, Calc X = {}, CalcY = {}", q_edge_offset, r_edge_offset, 
                        //calculated_offset.x, calculated_offset.y);

                    //let calculated_world_pos : Vec3 = Self::edgeAxialToWorld(q_edge_offset, r_edge_offset, world_position, is_north, is_west, is_east);
                    //println!("World X = {}, World Y = {}, Calc World X = {}, Calc World Y = {}", border_edge.x, border_edge.z, calculated_world_pos.x, calculated_world_pos.z);

                    match self.edges {
                        Some(ref mut edges) => {
                            if is_north {
                                edges[x_edge_index][y_edge_index].north = Some(Edge { world_coordinates: border_edge })
                            }
                            else if is_east {
                                edges[x_edge_index][y_edge_index].east = Some(Edge { world_coordinates: border_edge })
                            }
                            else if is_west {
                                edges[x_edge_index][y_edge_index].west = Some(Edge {world_coordinates : border_edge })
                            }
                        },
                        None => ()
                    }
                }

                match self.hexes{
                    Some(ref mut hexes) => {
                        hexes[x_index][y_index] = 
                            Some(Hex{
                                    center_coordinates : world_position
                                }
                            );
                    },
                    None => ()
                }
            }
        }
    }
}