use crate::common::*;

/// The number of characters in the game's statically-sized character array
pub const NUM_CHARACTERS: usize = 34;
/// The number of 3D game objects in the game's statically-sized object array
pub const NUM_OBJECTS: usize = 32;
/// The size of the Character structure representing an object-type "character"
pub const OBJECT_CHARACTER_SIZE: usize = 0x1F8;

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
    pub unk_x: Fixed16,        // 10
    pub unk_z: Fixed16,        // 12
    pub unk_y: Fixed16,        // 14
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
    pub flags: u32,                    // 000
    pub state: [u8; 4],                // 004
    pub id: u8,                        // 005
    pub unk_09: [u8; 0x3],             // 009
    pub index: u8,                     // 00C
    pub unk_0d: [u8; 0x17],            // 00D
    pub transform: MATRIX,             // 024
    pub pos_short: SVECTOR,            // 044
    pub base_pos_short: SVECTOR,       // 04C
    pub unk_54: [u8; 0x22],            // 054
    pub motion_angle: Fixed16,         // 076
    pub unk_78: [u8; 0xc],             // 078
    pub parts: [CharacterPart; 4],     // 084
    pub unk_104: u16,                  // 104
    pub floor: u8,                     // 106
    pub num_model_parts: u8,           // 107
    pub unk_108: [u8; 6],              // 108
    pub type_: u16,                    // 10E
    pub collision_state: u32,          // 110
    pub colliders_hit: u32,            // 114
    pub next_x: Fixed16,               // 118
    pub next_z: Fixed16,               // 11A
    pub unk_11c: [u8; 0x28],           // 11C
    pub velocity: SVECTOR,             // 144
    pub unk_14c: [u8; 0xa],            // 14C
    pub health: i16,                   // 156
    pub motion: i16,                   // 158
    pub unk_15a: [u8; 0x3e],           // 15A
    pub model_parts: Ptr32<ModelPart>, // 198
    pub unk_19c: [u8; 0x4C],           // 19C
    pub num_parts: u32,                // 1E8
    pub weapon_hit_stage_frames: u8,   // 1EC
    pub weapon_hit_stage_index: u8,    // 1ED
    pub unk_1ee: [u8; 0x2],            // 1EE
    pub distance_to_target: u32,       // 1F0
    pub unk_1f4: u32,                  // 1F4
    pub unk_1f8: u32,                  // 1F8
    pub prev_state: [u8; 4],           // 1FC
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