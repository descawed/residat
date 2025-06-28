use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::common::*;

/// The number of characters in the game's statically-sized character array
pub const NUM_CHARACTERS: usize = 34;
/// The number of 3D game objects in the game's statically-sized object array
pub const NUM_OBJECTS: usize = 32;
/// The size of the Character structure representing an object-type "character"
pub const OBJECT_CHARACTER_SIZE: usize = 0x1F8;
/// The maximum number of parts that a Character can be composed of.
pub const MAX_PARTS: usize = 4;

/// The ID of a particular character type
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CharacterId {
    Leon = 0,
    Claire = 1,
    Unknown2 = 2,
    Unknown3 = 3,
    LeonBandaged = 4,
    ClaireBlackTop = 5,
    Unknown6 = 6,
    Unknown7 = 7,
    LeonTankTop = 8,
    ClaireBiker = 9,
    LeonSkullJacket = 10,
    Chris = 11,
    Hunk = 12,
    Tofu = 13,
    Ada = 14,
    Sherry = 15,
    ZombiePoliceHat = 16,
    Brad = 17,
    ZombieTornShirt = 18,
    Misty = 19,
    Unknown20 = 20,
    ZombieLabWhite = 21,
    ZombieLabYellow = 22,
    NakedZombie = 23,
    ZombieYellowShirt = 24,
    Unknown25 = 25,
    Unknown26 = 26,
    Unknown27 = 27,
    Unknown28 = 28,
    Unknown29 = 29,
    HeadlessZombieYellowShirt = 30,
    ZombieRandom = 31,
    Dog = 32,
    Crow = 33,
    LickerRed = 34,
    Croc = 35,
    LickerBlack = 36,
    Spider = 37,
    SpiderBaby = 38,
    GYoung = 39,
    GAdult = 40,
    Roach = 41,
    MrX = 42,
    SuperX = 43,
    Unknown44 = 44,
    Hands = 45,
    Ivy = 46,
    Tentacle = 47,
    G1 = 48,
    G2 = 49,
    Unknown50 = 50,
    G3 = 51,
    G4 = 52,
    Unknown53 = 53,
    G5 = 54,
    G5Tentacle = 55,
    Unknown56 = 56,
    PoisonIvy = 57,
    Moth = 58,
    Larva = 59,
    Unknown60 = 60,
    Unknown61 = 61,
    FuseArm = 62,
    FuseHousing = 63,
    Irons = 64,
    AdaNpc = 65,
    IronsTorso = 66,
    AdaWounded = 67,
    BenDead = 68,
    SherryNpc = 69,
    Ben = 70,
    Annette = 71,
    Kendo = 72,
    Unknown73 = 73,
    Marvin = 74,
    MayorsDaughter = 75,
    Unknown76 = 76,
    Unknown77 = 77,
    Unknown78 = 78,
    SherryVest = 79,
    LeonNpc = 80,
    ClaireNpc = 81,
    Unknown82 = 82,
    Unknown83 = 83,
    LeonBandagedNpc = 84,
    Unknown = 255,
}

impl CharacterId {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Leon => "Leon",
            Self::Claire => "Claire",
            Self::Unknown2 => "Unknown 2",
            Self::Unknown3 => "Unknown 3",
            Self::LeonBandaged => "Leon (bandaged)",
            Self::ClaireBlackTop => "Claire (black top)",
            Self::Unknown6 => "Unknown 6",
            Self::Unknown7 => "Unknown 7",
            Self::LeonTankTop => "Leon (tank top)",
            Self::ClaireBiker => "Claire (biker)",
            Self::LeonSkullJacket => "Leon (skull jacket)",
            Self::Chris => "Chris",
            Self::Hunk => "Hunk",
            Self::Tofu => "Tofu",
            Self::Ada => "Ada",
            Self::Sherry => "Sherry",
            Self::ZombiePoliceHat => "Zombie (police hat)",
            Self::Brad => "Brad",
            Self::ZombieTornShirt => "Zombie (torn shirt)",
            Self::Misty => "Misty",
            Self::Unknown20 => "Unknown Zombie 20",
            Self::ZombieLabWhite => "Zombie (lab, white)",
            Self::ZombieLabYellow => "Zombie (lab, yellow)",
            Self::NakedZombie => "Naked zombie",
            Self::ZombieYellowShirt => "Zombie (yellow shirt)",
            Self::Unknown25 => "Unknown Zombie 25",
            Self::Unknown26 => "Unknown Zombie 26",
            Self::Unknown27 => "Unknown Zombie 27",
            Self::Unknown28 => "Unknown Zombie 28",
            Self::Unknown29 => "Unknown Zombie 29",
            Self::HeadlessZombieYellowShirt => "Headless zombie (yellow shirt)",
            Self::ZombieRandom => "Zombie (random)",
            Self::Dog => "Dog",
            Self::Crow => "Crow",
            Self::LickerRed => "Licker (red)",
            Self::Croc => "Croc",
            Self::LickerBlack => "Licker (black)",
            Self::Spider => "Spider",
            Self::SpiderBaby => "Baby spider",
            Self::GYoung => "G Young",
            Self::GAdult => "G Adult",
            Self::Roach => "Roach",
            Self::MrX => "Mr. X",
            Self::SuperX => "Tyrant",
            Self::Unknown44 => "Unknown 44",
            Self::Hands => "Hands",
            Self::Ivy => "Ivy",
            Self::Tentacle => "Tentacle",
            Self::G1 => "G1",
            Self::G2 => "G2",
            Self::Unknown50 => "Unknown 50",
            Self::G3 => "G3",
            Self::G4 => "G4",
            Self::Unknown53 => "Unknown 53",
            Self::G5 => "G5",
            Self::G5Tentacle => "G5 Tentacle",
            Self::Unknown56 => "Unknown 56",
            Self::PoisonIvy => "Poison Ivy",
            Self::Moth => "Moth",
            Self::Larva => "Larva",
            Self::Unknown60 => "Unknown 60",
            Self::Unknown61 => "Unknown 61",
            Self::FuseArm => "Fuse Arm",
            Self::FuseHousing => "Fuse Housing",
            Self::Irons => "Irons",
            Self::AdaNpc => "Ada (NPC)",
            Self::IronsTorso => "Irons (torso)",
            Self::AdaWounded => "Ada (wounded)",
            Self::BenDead => "Ben (dead)",
            Self::SherryNpc => "Sherry (NPC)",
            Self::Ben => "Ben",
            Self::Annette => "Annette",
            Self::Kendo => "Kendo",
            Self::Unknown73 => "Unknown 73",
            Self::Marvin => "Marvin",
            Self::MayorsDaughter => "Mayor's daughter",
            Self::Unknown76 => "Unknown 76",
            Self::Unknown77 => "Unknown 77",
            Self::Unknown78 => "Unknown 78",
            Self::SherryVest => "Sherry (vest)",
            Self::LeonNpc => "Leon (NPC)",
            Self::ClaireNpc => "Claire (NPC)",
            Self::Unknown82 => "Unknown 82",
            Self::Unknown83 => "Unknown 83",
            Self::LeonBandagedNpc => "Leon (bandaged, NPC)",
            Self::Unknown => "Unknown",
        }
    }

    pub const fn is_player(&self) -> bool {
        matches!(self, Self::Leon
            | Self::Claire
            | Self::Unknown2
            | Self::Unknown3
            | Self::LeonBandaged
            | Self::ClaireBlackTop
            | Self::Unknown6
            | Self::Unknown7
            | Self::LeonTankTop
            | Self::ClaireBiker
            | Self::LeonSkullJacket
            | Self::Chris
            | Self::Hunk
            | Self::Tofu
            | Self::Ada
            | Self::Sherry)
    }

    pub const fn is_zombie(&self) -> bool {
        matches!(self,
            Self::ZombiePoliceHat
            | Self::ZombieTornShirt
            | Self::ZombieYellowShirt
            | Self::ZombieRandom
            | Self::ZombieLabWhite
            | Self::ZombieLabYellow
            | Self::Misty
            | Self::Unknown20
            | Self::Unknown25
            | Self::Unknown26
            | Self::Unknown27
            | Self::Unknown28
            | Self::Unknown29
            | Self::Brad
            | Self::NakedZombie
            | Self::HeadlessZombieYellowShirt
        )
    }

    pub const fn is_licker(&self) -> bool {
        matches!(self, Self::LickerRed | Self::LickerBlack)
    }
}

/// 3D and other information for a part of a 3D model in the game world
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ModelPart {
    pub unk_00: u32,                     // 00
    pub unk_04: u32,                     // 04
    pub unk_08: u32,                     // 08
    pub model_base: u32,                 // 0C
    pub unk_10: u32,                     // 10
    pub unk_14: u32,                     // 14
    pub own_transform: MATRIX,           // 18
    pub unk_38: SSVECTOR,                // 38
    pub unk_3e: SSVECTOR,                // 3E
    pub unk_44: u32,                     // 44
    pub composite_transform: MATRIX,     // 48
    pub unk_68: u32,                     // 68
    pub unk_6c: u16,                     // 6C
    pub unk_6e: u16,                     // 6E
    pub unk_70: CVECTOR,                 // 70
    pub parent_transform: Ptr32<MATRIX>, // 74
    pub unk_78: u8,                      // 78
    pub unk_79: [u8; 13],                // 79
    pub unk_86: u16,                     // 86
    pub unk_88: u16,                     // 88
    pub unk_8a: u16,                     // 8A
    pub unk_8c: u16,                     // 8C
    pub unk_8e: u16,                     // 8E
    pub unk_90: u16,                     // 90
    pub unk_92: u16,                     // 92
    pub parent_flags: Ptr32<u32>,        // 94
    pub unk_98: [u16; 10],               // 98
}

/// 3D and other information for a part of a character in the game world
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CharacterPart {
    pub pos: VECTOR,           // 00
    pub x_size: UFixed16,      // 0C
    pub z_size: UFixed16,      // 0E
    pub local_x: Fixed16,      // 10
    pub local_z: Fixed16,      // 12
    pub local_y: Fixed16,      // 14
    pub size_offset: UFixed16, // 16
    pub unk_18: u16,           // 18
    pub y_size: UFixed16,      // 1A
    pub x_offset: Fixed16,     // 1C
    pub z_offset: Fixed16,     // 1E
}

/// A character (player, NPC, enemy, or certain 3D objects) in the game world
#[repr(C)]
#[derive(Debug)]
pub struct Character {
    pub flags: u32,                        // 000
    pub state: [u8; 4],                    // 004
    pub id: u8,                            // 005
    pub unk_09: [u8; 0x3],                 // 009
    pub index: u8,                         // 00C
    pub unk_0d: [u8; 0x17],                // 00D
    pub transform: MATRIX,                 // 024
    pub prev_transform_pos: SSVECTOR,      // 044
    pub prev_root_part_pos: SSVECTOR,      // 04A
    pub unk_50: [u8; 0x26],                // 050
    pub motion_angle: Fixed16,             // 076
    pub unk_78: [u8; 0xc],                 // 078
    pub parts: [CharacterPart; MAX_PARTS], // 084
    pub unk_104: u16,                      // 104
    pub floor: u8,                         // 106
    pub num_model_parts: u8,               // 107
    pub unk_108: [u8; 6],                  // 108
    pub type_: u16,                        // 10E
    pub collision_state: u32,              // 110
    pub colliders_hit: u32,                // 114
    pub prev_x: Fixed16,                   // 118
    pub prev_z: Fixed16,                   // 11A
    pub unk_11c: [u8; 0x28],               // 11C
    pub velocity: SVECTOR,                 // 144
    pub unk_14c: [u8; 0xa],                // 14C
    pub health: i16,                       // 156
    pub motion: i16,                       // 158
    pub unk_15a: [u8; 0x3e],               // 15A
    pub model_parts: Ptr32<ModelPart>,     // 198
    pub unk_19c: [u8; 0x4C],               // 19C
    pub num_parts: u32,                    // 1E8
    pub weapon_hit_stage_frames: u8,       // 1EC
    pub weapon_hit_stage_index: u8,        // 1ED
    pub unk_1ee: [u8; 0x2],                // 1EE
    pub distance_to_target: u32,           // 1F0
    pub unk_1f4: u32,                      // 1F4
    pub unk_1f8: u32,                      // 1F8
    pub prev_state: [u8; 4],               // 1FC
}

impl Character {
    pub unsafe fn model_parts(&self) -> &[ModelPart] {
        let parts_ptr = self.model_parts.ptr();
        if parts_ptr.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(parts_ptr, self.num_model_parts as usize) }
        }
    }

    pub fn parts(&self) -> &[CharacterPart] {
        &self.parts[..self.num_parts.max(1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use std::mem::offset_of;
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(size_of::<CharacterPart>(), 32);
        assert_eq!(size_of::<ModelPart>(), 0xAC);
    }

    #[test]
    fn test_layout() {
        assert_eq!(offset_of!(Character, parts), 0x84);
        assert_eq!(offset_of!(Character, floor), 0x106);
        assert_eq!(offset_of!(Character, type_), 0x10e);
        assert_eq!(offset_of!(Character, unk_11c), 0x11c);
        assert_eq!(offset_of!(Character, motion), 0x158);
        assert_eq!(offset_of!(Character, unk_15a), 0x15a);
        assert_eq!(offset_of!(Character, distance_to_target), 0x1f0);
        assert_eq!(offset_of!(Character, prev_state), 0x1fc);
    }
}