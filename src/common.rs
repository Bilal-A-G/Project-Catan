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
        Anything
    }

    pub fn IntToResourceType(integer : i8) -> ResourceType
    {
        match integer {
            1 => {return ResourceType::Wood;},
            2 => {return ResourceType::Stone;},
            3 => {return ResourceType::Brick;},
            4 => {return ResourceType::Sheep;},
            5 => {return ResourceType::Wheat;},
            _=> {println!("Int out of range!"); return ResourceType::Nothing;}
        }
    }

    #[repr(u8)]
    #[derive(Copy, Clone)]
    pub enum SettlementTier{
        Basic,
        City
    }
    
    #[derive(Copy, Clone)]
    pub struct PortData {
        pub input : ResourceType,
        pub num_inputs : i8
    }

    #[derive(Copy, Clone)]
    pub struct RoadData {
        pub player_id : i8
    }

    #[derive(Copy, Clone)]
    pub struct HexData {
        pub resource : ResourceType,
        pub dice_num : i8,
        pub has_robber : bool
    }

    #[derive(Copy, Clone)]
    pub struct SettlementData{
        pub player_id : i8,
        pub tier : SettlementTier
    }
}