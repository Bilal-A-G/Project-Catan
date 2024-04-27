use bevy::math::{Vec2, Vec3, vec3};
use bevy::ecs::system::{Resource, Res, Commands};
use bevy::transform::components::Transform;
use bevy::scene::{Scene, SceneBundle};
use bevy::asset::AssetServer;
use bevy::utils::default;

const MAP_HEIGHT : i8 = 10; 
const MAP_WIDTH : i8 = 10; 

const HEX_RADIUS : f32 = 2.0f32;
const J_OFFSET : f32 = HEX_RADIUS - 0.90f32;
const J_SPACING : f32 = HEX_RADIUS + 0.22f32;
const INITIAL_TRANSLATION : Vec3 = Transform::from_xyz(-4.0f32, 0.0f32, 7.0f32).translation;

const X_BASIS_VECTOR : Vec2 = Vec2::new(-HEX_RADIUS * 0.3f32, -HEX_RADIUS/2.0f32);
const Z_BASIS_VECTOR : Vec2 = Vec2::new (HEX_RADIUS * 0.3f32, HEX_RADIUS/2.0f32);

const VERTEX_TOO_FAR_AWAY: f32 = 0.5f32;

#[derive(Copy, Clone)]
pub struct Vertex 
{
    has_settlement : bool,
    is_hex_center : bool,
    world_coordinates : Vec3
}

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

    pub fn index_to_grid(i : i8, j : i8) -> Vec3
    {
        let grid_coords : Vec3 = Vec3::new(
            i as f32 + f32::floor(j as f32 / 2f32) + (j as f32 % 2f32) + 1f32, 
            j as f32, 
            f32::floor(j as f32 / 2f32) - i as f32 + MAP_WIDTH as f32/2f32
        );

        return grid_coords;
    }

    pub fn spawn(&mut self, command_queue : &mut Commands, asset_server : Res<AssetServer>)
    {
        let hexagon : bevy::prelude::Handle<Scene> = asset_server.load("Hex.glb#Scene0");
    
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
            let offset_z : f32 = INITIAL_TRANSLATION.z + if j % 2 == 0 {J_OFFSET} else {0.0f32};
            let offset_x : f32 = INITIAL_TRANSLATION.x + (HEX_RADIUS) * j as f32;
    
            for i in 0..MAP_WIDTH/2
            {
                command_queue.spawn(SceneBundle{
                    scene : hexagon.clone(),
                    transform : Transform::from_xyz(offset_x, INITIAL_TRANSLATION.y,  offset_z - i as f32 * J_SPACING),
                    ..default()
                });
                
                let center : Vec3 = vec3(offset_x, INITIAL_TRANSLATION.y,  offset_z - i as f32 * J_SPACING);
    
                let bottom : Vec3 = center + vec3(HEX_RADIUS/1.9f32, 0.0f32, 0.0f32);
                let top : Vec3 = center - vec3(HEX_RADIUS/1.9f32, 0.0f32, 0.0f32);
                let top_right : Vec3 = center - vec3(-X_BASIS_VECTOR.x, 0.0f32, -X_BASIS_VECTOR.y);
                let top_left : Vec3 = center - vec3(-X_BASIS_VECTOR.x, 0.0f32, X_BASIS_VECTOR.y);
                let bottom_right : Vec3 = center + vec3(Z_BASIS_VECTOR.x, 0.0f32, -Z_BASIS_VECTOR.y);
                let bottom_left : Vec3 = center + vec3(Z_BASIS_VECTOR.x, 0.0f32, Z_BASIS_VECTOR.y);
    
                let index_to_grid : Vec3 = Map::index_to_grid(i, j);
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
                        match vec[x - 1][y][z - 1] {
                            Some(_) => (),
                            None => {
                                vec[x - 1][y][z - 1] = Some(Vertex{has_settlement: false, world_coordinates: top, is_hex_center: false});
                            }
                        };
                        match vec[x][y][z - 1] {
                            Some(_) => (),
                            None => {
                                vec[x][y][z - 1] = Some(Vertex{has_settlement: false, world_coordinates: top_right, is_hex_center: false});
                            }
                        };
                        match vec[x - 1][y][z] {
                            Some(_) => (),
                            None => {
                                println!("x = {}, x - 1 = {}", x, x-1);
                                vec[x - 1][y][z] = Some(Vertex{has_settlement: false, world_coordinates: top_left, is_hex_center: false});
                            }
                        };
                        match vec[x][y + 1][z - 1] {
                            Some(_) => (),
                            None => {
                                vec[x][y + 1][z - 1] = Some(Vertex{has_settlement: false, world_coordinates: bottom_right, is_hex_center: false});
                            }
                        };
                        match vec[x - 1][y + 1][z] {
                            Some(_) => (),
                            None => {
                                vec[x - 1][y + 1][z] = Some(Vertex{has_settlement: false, world_coordinates: bottom_left, is_hex_center: false});
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

    pub fn try_place_settlement(&mut self, vertex_index: Vec3) -> bool
    {
        let max_x : i8 = MAP_WIDTH/2 + f32::floor((MAP_HEIGHT as f32/2f32) / 2f32) as i8;
        let max_z : i8 = max_x;
        let max_y : i8 = MAP_HEIGHT/2;

        match &mut self.vertices {
            Some(vertices) => {
                let top_neighbour: Vec3 = vertex_index + vec3(0f32, -1f32, 0f32);
                let bottom_neighbour: Vec3 = vertex_index + vec3(0f32, 1f32, 0f32);

                let z_left_neighbour: Vec3 = vertex_index + vec3(0f32, 0f32, -1f32);
                let z_right_neighbour: Vec3 = vertex_index + vec3(0f32, 0f32, 1f32);

                let x_left_neighbour: Vec3 = vertex_index + vec3(-1f32, 0f32, 0f32);
                let x_right_neighbour: Vec3 = vertex_index + vec3(1f32, 0f32, 0f32);

                let mut can_place: bool = true;
                for i in 0..6 {
                    let mut current_vertex_index : Vec3 = vertex_index;
                    match i {
                        0 => {current_vertex_index = top_neighbour},
                        1 =>{current_vertex_index = bottom_neighbour},
                        2 =>{current_vertex_index = z_left_neighbour},
                        3 =>{current_vertex_index = z_right_neighbour},
                        4 =>{current_vertex_index = x_left_neighbour},
                        5 =>{current_vertex_index = x_right_neighbour},
                        _=>()
                    };

                    if current_vertex_index.x < 0f32 || current_vertex_index.x > max_x as f32 || 
                    current_vertex_index.y < 0f32 || current_vertex_index.y > max_y as f32 ||
                    current_vertex_index.z < 0f32 || current_vertex_index.z > max_z as f32 
                    {
                        continue;
                    }

                    match vertices[current_vertex_index.x as usize][current_vertex_index.y as usize][current_vertex_index.z as usize] 
                    {
                        Some(neighbour_vertex) => {
                            if neighbour_vertex.is_hex_center {continue;}
                            println!("index: {} has settlement: {}", current_vertex_index, neighbour_vertex.has_settlement);
                            if can_place {
                                can_place = !neighbour_vertex.has_settlement;
                            }
                        },
                        None => ()
                    };
                }

                match &mut vertices[vertex_index.x as usize][vertex_index.y as usize][vertex_index.z as usize] {
                    Some(vertex) => {
                        if !can_place {return false;}

                        vertex.has_settlement = true;
                        return can_place;
                    },
                    None => ()
                }
            },
            None => ()
        }
        return false;
    }

    pub fn get_vertex_at_position(&self, x : f32, z : f32, select_centers: bool) -> Option<Vec3>
    {
        let mut closest_vertex_index: Vec3 = Vec3::new(0f32, 0f32, 0f32);
        let mut closest_vertex_distance: Option<f32> = None;

        match &self.vertices {
            Some(vertices) => {
                for i in 0..vertices.len() 
                {
                    for j in 0..vertices[i].len()  
                    {
                        for k in 0..vertices[i][j].len() 
                        {
                            match vertices[i][j][k] {
                                Some(vertex) => {
                                    let vertex_distance: f32 = 
                                    (vertex.world_coordinates - Vec3::new(x, vertex.world_coordinates.y, z)).length();

                                    match closest_vertex_distance {
                                        None => {
                                            closest_vertex_distance = Some(vertex_distance);
                                            closest_vertex_index = Vec3::new(i as f32, j as f32, k as f32);
                                        },
                                        Some(closest_distance) => {
                                            if vertex_distance < closest_distance 
                                            {
                                                closest_vertex_distance = Some(vertex_distance);
                                                closest_vertex_index = Vec3::new(i as f32, j as f32, k as f32);
                                            }
                                        }
                                    }
                                },
                                None => ()
                            }
                        }
                    }
                }

                match closest_vertex_distance {
                    Some(closest_distance) => { if closest_distance > VERTEX_TOO_FAR_AWAY {return Option::None;}},
                    None => ()
                }

                match vertices[closest_vertex_index.x as usize][closest_vertex_index.y as usize][closest_vertex_index.z as usize] {
                    Some(vertex) => {if !select_centers && vertex.is_hex_center {return Option::None}},
                    None => ()
                }
                return Option::Some(closest_vertex_index);
            },
            None => {return Option::None;}
        }
    }
}
