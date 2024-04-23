use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy::window::PresentMode;
use level::initialize_map;
use level::mouse_moved;
use level::spawn_camera;
use level::spawn_lights;
mod level;

pub const WIDTH : f32 = 600f32;
pub const HEIGHT : f32 = 400f32;

fn main()
{
    let map : level::Map = level::Map::create_new();

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1f32, 0.1f32, 0.1f32)))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
                primary_window : Some(Window{
                    resolution : (WIDTH, HEIGHT).into(),
                    title : "Bevy Project Catan".to_string(),
                    present_mode : PresentMode::AutoVsync,
                    resizable : true,
                    ..default()
                }),
                ..default()
            }))
        .insert_resource(map)
        .add_systems(Update, close_on_esc)
        .add_systems(Startup, spawn_lights) 
        .add_systems(Startup, spawn_camera) 
        .add_systems(Startup, initialize_map)
        .add_systems(Update, mouse_moved)
        .run(); 
}