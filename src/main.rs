use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy::window::PresentMode;
use level::create_test_scene;
use level::spawn_camera;
mod level;

pub const WIDTH : f32 = 600f32;
pub const HEIGHT : f32 = 400f32;

fn main()
{
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1f32, 0.1f32, 0.1f32)))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
                primary_window : Some(Window{
                    resolution : (WIDTH, HEIGHT).into(),
                    title : "Bevy Project Catan".to_string(),
                    present_mode : PresentMode::AutoVsync,
                    resizable : false,
                    ..default()
                }),
                ..default()
            }))
        .add_systems(Update, close_on_esc)
        .add_systems(Startup, create_test_scene) 
        .add_systems(Startup, spawn_camera) 
        .run();  
}