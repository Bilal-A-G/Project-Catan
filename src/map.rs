use bevy::math::{vec2, vec3, Vec2, Vec3};
use bevy::ecs::system::{Resource, Res, Commands};
use bevy::render::render_resource::encase::rts_array::Length;
use bevy::transform::components::Transform;
use bevy::scene::{Scene, SceneBundle};
use bevy::asset::{self, AssetServer};
use bevy::utils::default;

use crate::common;
use crate::common::common::{HexData, PortData, RoadData, SettlementData};
use crate::common::common::ResourceType;
use rand::Rng;
const MAP_SIZE : i8 = 2; 

const HEX_RADIUS : f32 = 1.0f32;
const DEFAULT_HEX_SIZE : f32 = 0.2f32;
const INITIAL_TRANSLATION : Vec3 = Transform::from_xyz(0.0f32, 0.0f32, 0.0f32).translation;

const HEX_HEIGHT : f32 = (HEX_RADIUS + DEFAULT_HEX_SIZE) * 2f32;

const HORIZONTAL_DISTANCE : f32 = 1.732f32 * (HEX_RADIUS + DEFAULT_HEX_SIZE);
const VERTICAL_DISTANCE : f32 = 3f32/4f32 * HEX_HEIGHT;

const CLOSENESS_THRESHOLD : f32 = 0.3f32;

#[derive(Copy, Clone)]
pub struct Vertex 
{
    pub world_coordinates : Vec3,
    pub port_data : Option<PortData>,
    pub settlement_data : Option<SettlementData>
}

pub struct PortPosition
{
    axial_coordinates : Vec2,
    is_bottom : bool
}

#[derive(Copy, Clone)]
pub struct Edge 
{
    pub world_coordinates : Vec3,
    pub road_data : Option<RoadData>
}

#[derive(Clone)]
pub struct HexVertex 
{
    pub top : Option<Vertex>,
    pub bottom : Option<Vertex>,
    pub center : Vec3
}

#[derive(Copy, Clone)]
pub struct HexEdge
{
    pub north : Option<Edge>,
    pub west : Option<Edge>,
    pub east : Option<Edge>
}

#[derive(Copy, Clone)]
pub struct Hex
{
    pub center_coordinates : Vec3,
    pub hex_data : HexData
}

#[derive(Resource)]
pub struct Map
{
    pub hexes : Vec<Vec<Option<Hex>>>,
    pub vertices : Vec<Vec<HexVertex>>,
    pub edges : Vec<Vec<HexEdge>>,

    ports : Vec<PortData>,
    port_vertices : Vec<PortPosition>
}

impl Map 
{
    pub fn create_new() -> Map
    {
        Map 
        { 
            hexes: vec![
                vec![None; (MAP_SIZE * 2 + 1) as usize]; 
                (MAP_SIZE * 2 + 1) as usize
            ],
            vertices: vec![
                vec![HexVertex{top : None, bottom : None, center : vec3(0f32, 0f32, 0f32)}; ((MAP_SIZE * 2 + 1) + 2) as usize];
                ((MAP_SIZE * 2 + 1) + 2) as usize               
            ],
            edges: vec![
                vec![HexEdge{north : None, west : None, east : None}; ((MAP_SIZE * 2 + 1) + 2) as usize];
                ((MAP_SIZE * 2 + 1) + 2) as usize
            ],
            ports: vec![
                PortData{input: ResourceType::Wheat, num_inputs : 2},
                PortData{input: ResourceType::Anything, num_inputs : 3}, 
                PortData{input: ResourceType::Wood, num_inputs : 2}, 
                PortData{input: ResourceType::Brick, num_inputs : 2},
                PortData{input: ResourceType::Anything, num_inputs : 3},
                PortData{input: ResourceType::Anything, num_inputs : 3}, 
                PortData{input: ResourceType::Sheep, num_inputs : 2},
                PortData{input: ResourceType::Anything, num_inputs : 3},
                PortData{input: ResourceType::Stone, num_inputs : 2}
                ],
            port_vertices: vec![
                PortPosition{axial_coordinates : vec2(1f32, -2f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(1f32, -3f32), is_bottom: true},

                PortPosition{axial_coordinates : vec2(2f32, -2f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(3f32, -3f32), is_bottom: true},

                PortPosition{axial_coordinates : vec2(3f32, -2f32), is_bottom: true},
                PortPosition{axial_coordinates : vec2(2f32, 0f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(2f32, 0f32), is_bottom: true},
                PortPosition{axial_coordinates : vec2(1f32, 2f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(0f32, 3f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(0f32, 2f32), is_bottom: true},

                PortPosition{axial_coordinates : vec2(-2f32, 1f32), is_bottom: true},
                PortPosition{axial_coordinates : vec2(-3f32, 2f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(-1f32, -2f32), is_bottom: true},
                PortPosition{axial_coordinates : vec2(-1f32, -1f32), is_bottom: false},

                PortPosition{axial_coordinates : vec2(-3f32, 1f32), is_bottom: false},
                PortPosition{axial_coordinates : vec2(-2f32, -1f32), is_bottom: true},

                PortPosition{axial_coordinates : vec2(-1f32, 2f32), is_bottom: true},
                PortPosition{axial_coordinates : vec2(-2f32, 3f32), is_bottom: false}
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

    //Returns a clone of the vertex in array, you cannot modify it
    pub fn getVertexFromAxial(&self, axial : Vec2, is_bottom : bool) -> Option<Vertex>
    {
        let vertex_index : Vec2 = axial + vec2(MAP_SIZE as f32 + 1f32, MAP_SIZE as f32 + 1f32);
        if (vertex_index.x < 0f32 || vertex_index.x > (self.vertices.len() - 1) as f32) || 
            (vertex_index.y < 0f32 || vertex_index.y > (self.vertices.len() - 1) as f32)
        {
            return None;
        }

        let hex_vertex = &self.vertices[vertex_index.x as usize][vertex_index.y as usize];
        if is_bottom {
            return hex_vertex.bottom;
        }
        else {
            return hex_vertex.top;
        }
    }

    pub fn getHexFromAxial(&self, axial : Vec2) -> Option<Hex>
    {
        let hex_index : Vec2 = axial + vec2(MAP_SIZE as f32, MAP_SIZE as f32);

        if (hex_index.x < 0f32 || hex_index.x > (self.hexes.len() - 1) as f32) || 
        (hex_index.y < 0f32 || hex_index.y > (self.hexes.len() - 1) as f32)
        {
            return None;
        }

        let hex : Option<Hex> = self.hexes[hex_index.x as usize][hex_index.y as usize];
        return hex;
    }

    pub fn getEdgeFromAxial(&self, axial : Vec2, is_north : bool, is_west : bool, is_east : bool) -> Option<Edge>
    {
        let edge_index : Vec2 = axial + vec2(MAP_SIZE as f32 + 1f32, MAP_SIZE as f32 + 1f32);
        if (edge_index.x < 0f32 || edge_index.x > (self.edges.len() - 1) as f32) || 
        (edge_index.y < 0f32 || edge_index.y > (self.edges.len() - 1) as f32)
        {
            return None;
        }

        let edge : HexEdge = self.edges[edge_index.x as usize][edge_index.y as usize];
        if is_north {
            return edge.north;
        }
        else if is_east {
            return edge.east;
        }
        else {
            return edge.west;
        }
    }

    pub fn getVertexNeighbourAxials(&self, axial : Vec2, is_bottom : bool) -> Vec<(Vec2, bool)>
    {
        let mut neighbours : Vec<(Vec2, bool)> = Vec::new();
        let mut offsets : Vec<Vec2> = Vec::new();
        offsets.push(vec2(1.0f32, -2.0f32));
        offsets.push(vec2(0.0f32, -1.0f32));
        offsets.push(vec2(1.0f32, -1.0f32));

        for i in 0 .. offsets.len() {
            let multiplier : f32 = if is_bottom {-1 as f32} else {1 as f32};
            let neighbour_offset : Vec2 = axial + offsets[i] * multiplier;
            let neighbour_vertex : Option<Vertex> = Self::getVertexFromAxial(&self, neighbour_offset, !is_bottom);
            match neighbour_vertex {
                Some(_) => {
                    neighbours.push((neighbour_offset, !is_bottom));
                },
                None => ()
            }
        }

        return neighbours; 
    }

    pub fn getVertexTouchingHexAxials(&self, axial : Vec2, is_bottom : bool) -> Vec<Vec2>
    {
        let mut touching_hexes : Vec<Vec2> = Vec::new();
        let mut offsets : Vec<Vec2> = Vec::new();
        offsets.push(vec2(1f32, -1f32));
        offsets.push(vec2(0f32, 0f32));
        offsets.push(vec2(0f32, -1f32));

        for i in 0..offsets.len() {
            let multiplier : f32 = if is_bottom {-1 as f32} else {1 as f32};
            let touching_hex_offset : Vec2 = axial + offsets[i] * multiplier;
            let touching_hex : Option<Hex> = self.getHexFromAxial(touching_hex_offset);
            match touching_hex {
                Some(_) => {
                    touching_hexes.push(touching_hex_offset);
                },
                None => ()
            }
        }

        return touching_hexes;
    }

    pub fn getVertexProtrudingEdgeAxials(&self, axial : Vec2, is_bottom : bool) -> Vec<(Vec2, bool, bool, bool)>
    {
        let mut protruding_edges : Vec<(Vec2, bool, bool, bool)> = Vec::new();
        let north_edge;
        let west_edge;
        let east_edge;

        if is_bottom {
            north_edge = (axial + vec2(0f32, 1f32), false, false, true);
            west_edge = (axial + vec2(-1f32, 1f32), false, true, false);
            east_edge = (axial+ vec2(-1f32, 1f32), true, false, false);
        }
        else {
            north_edge = (axial + vec2(0f32, -1f32), false, true, false);
            west_edge = (axial, true, false, false);
            east_edge = (axial, false, false, true);
        }

        let protruding_north_edge = 
            self.getEdgeFromAxial(north_edge.0, north_edge.1, north_edge.2, north_edge.3);
        let protruding_west_edge = 
            self.getEdgeFromAxial(west_edge.0, west_edge.1, west_edge.2, west_edge.3);
        let protruding_east_edge = 
            self.getEdgeFromAxial(east_edge.0, east_edge.1, east_edge.2, east_edge.3);

        match protruding_north_edge {
            Some(_) => {
                protruding_edges.push(north_edge);
            },
            None => ()
        }
        match protruding_west_edge {
            Some(_) => {
                protruding_edges.push(west_edge);
            },
            None => ()
        }
        match protruding_east_edge {
            Some(_) => {
                protruding_edges.push(east_edge);
            },
            None => ()
        }
        return protruding_edges;
    }

    pub fn getEdgeNeighbouringEdgeAxials(&self, axial : Vec2, is_north : bool, is_west : bool, is_east : bool) -> Vec<(Vec2, bool, bool, bool)>
    {
        let mut neighbouring_edges : Vec<(Vec2, bool, bool, bool)> = Vec::new();
        let top_left_edge;
        let top_right_edge;
        let bottom_left_edge;
        let bottom_right_edge;

        if is_north {
            top_left_edge = (axial + vec2(1f32, 0f32), false, false, true);
            top_right_edge = (axial + vec2(0f32, 0f32), false, true, false);
            bottom_left_edge = (axial + vec2(0f32, 0f32), false, false, true);
            bottom_right_edge = (axial + vec2(0f32, -1f32), false, true, false);
        }
        else if is_west{
            top_left_edge = (axial + vec2(0f32, 0f32), true, false, false);
            top_right_edge = (axial + vec2(1f32, 0f32), false, false, true);
            bottom_left_edge = (axial + vec2(0f32, 1f32), true, false, false);
            bottom_right_edge = (axial + vec2(0f32, 1f32), false, false, true);
        }
        else {
            top_left_edge = (axial + vec2(0f32, 0f32), true, false, false);
            top_right_edge = (axial + vec2(0f32, -1f32), false, true, false);
            bottom_left_edge = (axial + vec2(-1f32, 0f32), true, false, false);
            bottom_right_edge = (axial + vec2(-1f32, 0f32), false, true, false);
        }

        let top_left = 
        self.getEdgeFromAxial(top_left_edge.0, top_left_edge.1, top_left_edge.2, top_left_edge.3);
        let top_right = 
        self.getEdgeFromAxial(top_right_edge.0, top_right_edge.1, top_right_edge.2, top_right_edge.3);
        let bottom_left = 
        self.getEdgeFromAxial(bottom_left_edge.0, bottom_left_edge.1, bottom_left_edge.2, bottom_left_edge.3);
        let bottom_right = 
        self.getEdgeFromAxial(bottom_right_edge.0, bottom_right_edge.1, bottom_right_edge.2, bottom_right_edge.3);

        match top_left {
            Some(_) => {
                neighbouring_edges.push(top_left_edge);
            },
            None => ()
        }
        match top_right {
            Some(_) => {
                neighbouring_edges.push(top_right_edge);
            },
            None => ()
        }
        match bottom_left {
            Some(_) => {
                neighbouring_edges.push(bottom_left_edge);
            },
            None => ()
        }
        match bottom_right {
            Some(_) => {
                neighbouring_edges.push(bottom_right_edge);
            },
            None => ()
        }
        return neighbouring_edges;
    }

    pub fn getEdgeEndPointAxials(&self, axial : Vec2, is_north : bool, is_west : bool, is_east : bool) -> Vec<(Vec2, bool)> 
    {
        let mut end_points : Vec<(Vec2, bool)> = Vec::new();
        let mut start_point : (Vec2, bool);
        let mut end_point : (Vec2, bool);

        if is_north {
            start_point = (axial + vec2(1f32, -1f32), true);
            end_point = (axial + vec2(0f32, 0f32), false); 
        }
        else if is_west {
            start_point = (axial + vec2(1f32, -1f32), true);
            end_point = (axial + vec2(0f32, 1f32), false); 
        }
        else {
            start_point = (axial + vec2(0f32, 0f32), false);
            end_point = (axial + vec2(0f32, -1f32), true); 
        }

        let end : Option<Vertex> = self.getVertexFromAxial(end_point.0, end_point.1);
        let start : Option<Vertex> = self.getVertexFromAxial(start_point.0, start_point.1);

        match end {
            Some(_) => {end_points.push(end_point);},
            None => ()
        }
        match start {
            Some(_) => {end_points.push(start_point);},
            None => ()
        }
        return end_points;
    }

    pub fn hexWorldToAxial(world : Vec3) -> Option<Vec2>
    {
        let y_axial : f32 = (world.x - INITIAL_TRANSLATION.x) / VERTICAL_DISTANCE;
        let x_axial : f32 = (world.z - INITIAL_TRANSLATION.z - y_axial * (HORIZONTAL_DISTANCE / 2f32)) / HORIZONTAL_DISTANCE;
        let hex_axial_rounded : Vec2 = Self::hexAxialRound(vec2(x_axial, y_axial));
        if Self::cubeToDist(Self::hexToCube(hex_axial_rounded.x as i8, hex_axial_rounded.y as i8)) > MAP_SIZE as f32 {
            return None;
        }
        return Some(hex_axial_rounded);
    }

    pub fn cubeToDist(cube : Vec3) -> f32
    {
        return (f32::abs(cube.x) + f32::abs(cube.y) + f32::abs(cube.z))/2f32;
    }

    pub fn vertexWorldToAxial(world : Vec3) -> Option<(Vec2, bool)>
    {
        let hex_axial : Option<Vec2> = Self::hexWorldToAxial(world);
        match hex_axial {
            Some(axial) => {
                let hex_cube_coords : Vec3 = Self::hexToCube(axial.x as i8, axial.y as i8);
                let distance_from_center : f32 = Self::cubeToDist(hex_cube_coords);
        
                if distance_from_center > MAP_SIZE as f32 {
                    return None;
                }
        
                let center : Vec3 = Self::hexAxialToWorld(axial.x as i8, axial.y as i8);
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
                return Some((vec2(q_offset as f32, r_offset as f32) + axial, vertex_i % 2 == 0));
            },
            None => {return None;}
        }
    }

    pub fn hexToCube(q : i8, r : i8) -> Vec3
    {
        return vec3((r + q) as f32, -r as f32, q as f32);
    }

    pub fn edgeWorldToAxial(world : Vec3) -> Option<(Vec2, bool, bool, bool)>
    {
        let hex_frac_axial : Option<Vec2> = Self::hexWorldToAxial(world);
        match hex_frac_axial {
            Some(axial) => {
                let rounded_hex_axial : Vec2 = Self::hexAxialRound(axial);
                let hex_cube_coords : Vec3 = Self::hexToCube(rounded_hex_axial.x as i8, rounded_hex_axial.y as i8);
                let distance_from_center : f32 = Self::cubeToDist(hex_cube_coords);
        
                if distance_from_center > MAP_SIZE as f32 {
                    return None;
                }
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
            },
            None => {return None;}
        }
    }

    pub fn hexAxialToWorld(q_offset : i8, r_offset : i8) -> Vec3 
    {
        let x_position : f32 = INITIAL_TRANSLATION.x + r_offset as f32 * VERTICAL_DISTANCE;
        let z_position : f32 = (INITIAL_TRANSLATION.z + q_offset as f32 * HORIZONTAL_DISTANCE) + 
            r_offset as f32 * (HORIZONTAL_DISTANCE / 2f32);
        
        return vec3(x_position, INITIAL_TRANSLATION.y, z_position);
    }

    pub fn vertexAxialToWorld(q_offset : i8, r_offset : i8, center : Vec3, is_bottom : bool ) -> Option<Vec3> 
    {
        let hex_axial : Option<Vec2> = Self::hexWorldToAxial(center);
        match hex_axial {
            Some(axial) => {
                let vertex_q_offset : i8 = q_offset - axial.x as i8;
                let vertex_r_offset : i8 = r_offset - axial.y as i8;
        
                let world_position : Vec3 = Self::getCorners(center, 
                    Self::vertexIFromOffset(vertex_q_offset, vertex_r_offset, is_bottom));
                
                return Some(world_position);
            },
            None => {return None}
        }
    }

    pub fn edgeAxialToWorld(q_offset : i8, r_offset : i8, center : Vec3, is_north : bool, is_west : bool, is_east : bool) -> Option<Vec3>
    {
        let hex_axial : Option<Vec2> = Self::hexWorldToAxial(center);
        match hex_axial {
            Some(axial) => {
                let edge_q_offset : i8 = q_offset - axial.x as i8;
                let edge_r_offset : i8 = r_offset - axial.y as i8;
        
                let world_position : Vec3 = Self::getEdges(center, 
                    Self::edgeIFromOffset(edge_q_offset, edge_r_offset, is_north, is_west, is_east));
        
                return Some(world_position);
            },
            None => {return None;}
        }
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
        if i == 2 || i == 3 || i == 4 {
            return 0;
        }
        if i == 0{
            return -1;
        }
        else if i == 1{
            return 0;
        }
        else {
            return -1;
        }
    }

    pub fn edgeROffsetFromI(i : i8) -> i8 
    {
        if i == 5 || i == 2 || i == 4 || i == 3 {
            return 0;
        }
        if i == 0 || i == 1 {
            return 1;
        }
        else {
            println!("Edge R cannot be found from I!");
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
                    let mut port_data : Option<PortData> = None;

                    for j in 0 .. self.port_vertices.length() {
                        let port_position : &PortPosition = &self.port_vertices[j];
                        if (q_vertex_offset as f32 == port_position.axial_coordinates.x) && 
                        (r_vertex_index as f32 == port_position.axial_coordinates.y) {
                            if port_position.is_bottom == is_bottom {
                                let mut index : usize = 0;
                                if j % 2 == 0 {
                                    index = (j / 2) as usize;
                                }
                                else {
                                    index = ((j - 1) / 2) as usize;
                                }
                                port_data = Some(self.ports[index]);
                            }
                        }
                    }
                    self.vertices[x_vertex_index][y_vertex_index].center = world_position;
                    if is_bottom {
                        self.vertices[x_vertex_index][y_vertex_index].bottom = 
                            Some(Vertex{world_coordinates : corner_vertex, port_data : port_data, settlement_data : None});
                    }
                    else {
                        self.vertices[x_vertex_index][y_vertex_index].top = 
                            Some(Vertex{world_coordinates : corner_vertex, port_data : port_data, settlement_data : None});
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

                    if is_north {
                        self.edges[x_edge_index][y_edge_index].north = Some(Edge { world_coordinates: border_edge, road_data: None })
                    }
                    else if is_east {
                        self.edges[x_edge_index][y_edge_index].east = Some(Edge { world_coordinates: border_edge, road_data: None })
                    }
                    else if is_west {
                        self.edges[x_edge_index][y_edge_index].west = Some(Edge {world_coordinates : border_edge, road_data: None })
                    }
                }

                let has_robber : bool = q_offset == 0 && r_offset == 0;
                let mut resource_type : ResourceType = 
                    common::common::IntToResourceType(rand::thread_rng().gen_range(1..ResourceType::Anything as i8));
                if q_offset == 0 && r_offset == 0 {
                    resource_type = ResourceType::Nothing;
                }
                let dice_a : i8 = rand::thread_rng().gen_range(1..7);
                let dice_b : i8 = rand::thread_rng().gen_range(1..7);

                self.hexes[x_index][y_index] = 
                Some(Hex{
                        center_coordinates : world_position,
                        hex_data : HexData{
                            resource : resource_type,
                            dice_num : (dice_a + dice_b),
                            has_robber : has_robber
                        }
                    }
                );
            }
        }
    }
}