use bevy::math::Vec3;

#[repr(u8)]
enum Resources
{
    Wood,
    Stone,
    Brick,
    Sheep,
    Wheat
}

#[repr(u8)]
enum Tiles
{
    Resource,
    Desert,
    Water   
}