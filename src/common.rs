use bevy::math::Vec3;

pub mod common 
{
    #[repr(u8)]
    #[derive(Copy, Clone)]
    pub enum ResourceType{
        Nothing,
        Wood,
        Stone,
        Brick,
        Sheep,
        Wheat,
        Anything,
        Water
    }
    
    #[derive(Copy, Clone)]
    pub struct PortData {
        pub input : ResourceType,
        pub num_inputs : i8
    }
}