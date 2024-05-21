use bevy::math::{vec2, vec3, Vec2, Vec3};
use bevy::ecs::system::{Resource, Res, Commands};
use bevy::transform::components::Transform;
use bevy::scene::{Scene, SceneBundle};
use bevy::asset::{self, AssetServer};
use bevy::utils::default;

const MAP_HEIGHT : i8 = 1; 
const MAP_WIDTH : i8 = 1; 

const HEX_RADIUS : f32 = 1.0f32;
const DEFAULT_HEX_SIZE : f32 = 0.2f32;
const INITIAL_TRANSLATION : Vec3 = Transform::from_xyz(0.0f32, 0.0f32, 0.0f32).translation;

const HEX_HEIGHT : f32 = (HEX_RADIUS + DEFAULT_HEX_SIZE) * 2f32;

const HORIZONTAL_DISTANCE : f32 = 1.732f32 * (HEX_RADIUS + DEFAULT_HEX_SIZE);
const VERTICAL_DISTANCE : f32 = 3f32/4f32 * HEX_HEIGHT;

#[derive(Copy, Clone)]
pub struct Vertex 
{
    world_coordinates : Vec3
}

#[derive(Copy, Clone)]
pub struct HexVertex 
{
    top : Option<Vertex>,
    bottom : Option<Vertex>
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
    vertices : Option<Vec<Vec<HexVertex>>>
}

impl Map 
{
    pub fn create_new() -> Map
    {
        Map 
        { 
            hexes: Some(vec![
                vec![None; (MAP_WIDTH * 2 + 1) as usize]; 
                (MAP_HEIGHT * 2 + 1) as usize
            ]),
            vertices: Some(vec![
                vec![HexVertex{top : None, bottom : None}; ((MAP_WIDTH * 2 + 1) + 2) as usize];
                ((MAP_HEIGHT * 2 + 1) + 2) as usize               
            ])
        }
    }

    pub fn hexAxialToWorld(q_offset : i8, r_offset : i8) -> Vec3 
    {
        let x_position : f32 = INITIAL_TRANSLATION.x + r_offset as f32 * VERTICAL_DISTANCE;
        let z_position : f32 = (INITIAL_TRANSLATION.z + q_offset as f32 * HORIZONTAL_DISTANCE) + 
            r_offset as f32 * (HORIZONTAL_DISTANCE / 2f32);
        
        return vec3(x_position, INITIAL_TRANSLATION.y, z_position);
    }

    pub fn vertexAxialToWorld(q_offset : i8, r_offset : i8, center : Vec3, isbottom : bool, ) -> Vec3 
    {
        let hex_axial : Vec2 = Self::hexWorldToAxial(center);
        let vertex_q_offset : i8 = hex_axial.x as i8 - q_offset;
        let vertex_r_offset : i8 = hex_axial.y as i8 - r_offset;

        let world_position : Vec3 = Self::getCorners(center, 
            Self::vertexIFromOffset(vertex_q_offset, vertex_r_offset, isbottom));
        
        return world_position;
    }

    pub fn vertexQOffsetFromI(i : i32) -> i8
    {
        if i == 0 || i == 3 || i == 2 || i == 5 {
            return 0;
        }

        if i % 2 == 0{
            return 1;
        }
        else {
            return -1;
        }
    }

    pub fn vertexIFromOffset(q_offset : i8, r_offset : i8, isbottom : bool) -> i32 
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
 
    pub fn vertexROffsetFromI(i : i32) -> i8
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
 
    pub fn hexWorldToAxial(world : Vec3) -> Vec2
    {
        let y_axial : f32 = (world.x - INITIAL_TRANSLATION.x) / VERTICAL_DISTANCE;
        let x_axial : f32 = (world.z - INITIAL_TRANSLATION.z - y_axial * (HORIZONTAL_DISTANCE / 2f32)) / HORIZONTAL_DISTANCE;
        return vec2(x_axial, y_axial);
    }

    pub fn getCorners(center : Vec3, i : i32) -> Vec3
    {
        let degree_angle : f32 = 30f32 + (60f32 * i as f32 - 30f32);
        let rad_angle : f32 = 3.14f32 / 180f32 * degree_angle;
        let vertex_coord : Vec3 = vec3(
            center.x + (HEX_RADIUS) * f32::cos(rad_angle), 
            center.y,
            center.z + (HEX_RADIUS) * f32::sin(rad_angle));
        
        return vertex_coord;
    }

    pub fn spawn(&mut self, command_queue : &mut Commands, asset_server : Res<AssetServer>) 
    {
        let hexagon : bevy::prelude::Handle<Scene> = asset_server.load("Hex.glb#Scene0");
        
        for q_offset in -MAP_WIDTH .. MAP_WIDTH + 1  
        {
            let mut r_lower_bounds : i8 = -MAP_HEIGHT - q_offset;
            let mut r_upper_bounds : i8 = MAP_HEIGHT - q_offset;

            r_lower_bounds = i8::clamp(r_lower_bounds, -MAP_HEIGHT, MAP_HEIGHT);
            r_upper_bounds = i8::clamp(r_upper_bounds, -MAP_HEIGHT, MAP_HEIGHT);

            for r_offset in r_lower_bounds .. r_upper_bounds + 1
            {
                let world_position : Vec3 = Self::hexAxialToWorld(q_offset, r_offset);

                command_queue.spawn(SceneBundle{
                    scene : hexagon.clone(),
                    transform : Transform::from_translation(world_position).
                        with_scale(vec3(HEX_RADIUS, HEX_RADIUS, HEX_RADIUS)),
                    ..default()
                });

                let x_index : usize = (q_offset + MAP_WIDTH) as usize;
                let y_index : usize = (r_offset + MAP_HEIGHT) as usize;

                println!("Coords of hex = x:{}, y:{}", q_offset, r_offset);
                for i in 0..6 {
                    let corner_vertex : Vec3 = Self::getCorners(world_position, i);
                    let q_vertex_offset : i8 = q_offset + Self::vertexQOffsetFromI(i);
                    let r_vertex_index : i8 = r_offset + Self::vertexROffsetFromI(i);
                    let is_bottom : bool = i % 2 == 0;

                    let x_vertex_index : usize = (q_vertex_offset + MAP_WIDTH + 1) as usize; 
                    let y_vertex_index : usize = (r_vertex_index + MAP_HEIGHT + 1) as usize; 

                    println!("X = {}, Y = {}, isBottom = {}", q_vertex_offset, r_vertex_index, is_bottom);

                    match self.vertices {
                        Some(ref mut vertices) => {
                            if is_bottom {
                                vertices[x_vertex_index][y_vertex_index].bottom = 
                                    Some(Vertex{world_coordinates : corner_vertex});
                            }
                            else {
                                vertices[x_vertex_index][y_vertex_index].top = 
                                    Some(Vertex{world_coordinates : corner_vertex});
                            }
                        }
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
