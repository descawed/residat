use std::io::{Cursor, Read, Seek, SeekFrom};

use anyhow::{Context, Result};
use binrw::{binrw, BinReaderExt};

use crate::common::*;
use super::script::Instruction;

#[binrw]
#[derive(Debug)]
struct RdtHeader {
    n_sprite: u8,
    n_cut: u8,
    o_model: u8,
    n_item: u8,
    n_door: u8,
    n_room_at: u8,
    reverb_lv: u8,
    n_sprite_max: u8,
    // section offsets
    sound_attr_offset: u32,
    sound_header1_offset: u32,
    sound_bank1_offset: u32,
    sound_header2_offset: u32,
    sound_bank2_offset: u32,
    ota_offset: u32,
    collision_offset: u32,
    camera_pos_offset: u32,
    camera_zone_offset: u32,
    light_offset: u32,
    model_offset: u32,
    floor_offset: u32,
    block_offset: u32,
    jp_message_offset: u32,
    other_message_offset: u32,
    camera_scroll_offset: u32,
    init_script_offset: u32,
    exec_script_offset: u32,
    sprite_id_offset: u32,
    sprite_data_offset: u32,
    sprite_texture_offset: u32,
    model_texture_offset: u32,
    animation_offset: u32,
}

impl RdtHeader {
    fn init_script_size(&self) -> usize {
        if self.init_script_offset == 0 {
            return 0;
        }

        let next_offset = if self.exec_script_offset > 0 {
            self.exec_script_offset
        } else if self.sprite_id_offset > 0 {
            self.sprite_id_offset
        } else if self.sprite_data_offset > 0 {
            self.sprite_data_offset
        } else if self.sprite_texture_offset > 0 {
            self.sprite_texture_offset
        } else if self.model_texture_offset > 0 {
            self.model_texture_offset
        } else if self.animation_offset > 0 {
            self.animation_offset
        } else {
            return 0;
        };

        (next_offset - self.init_script_offset) as usize
    }

    fn exec_script_size(&self) -> usize {
        if self.exec_script_offset == 0 {
            return 0;
        }

        let next_offset = if self.sprite_id_offset > 0 {
            self.sprite_id_offset
        } else if self.sprite_data_offset > 0 {
            self.sprite_data_offset
        } else if self.sprite_texture_offset > 0 {
            self.sprite_texture_offset
        } else if self.model_texture_offset > 0 {
            self.model_texture_offset
        } else if self.animation_offset > 0 {
            self.animation_offset
        } else {
            return 0;
        };

        (next_offset - self.exec_script_offset) as usize
    }
}

#[binrw]
#[derive(Debug)]
pub struct Collider {
    pub x: Fixed16,
    pub z: Fixed16,
    pub w: UFixed16,
    pub h: UFixed16,
    pub packed: u32,
    pub floor: u32,
}

impl Collider {
    pub const fn collision_mask(&self) -> u16 {
        (self.packed & 0xfff0) as u16
    }
}

#[binrw]
#[derive(Debug)]
pub struct Collision {
    pub cell_x: Fixed16,
    pub cell_z: Fixed16,
    count: u32,
    pub ceiling: i32,
    pub dummy: u32,
    #[br(count = count - 1)]
    pub colliders: Vec<Collider>,
}

impl Default for Collision {
    fn default() -> Self {
        Self {
            cell_x: Fixed16(0),
            cell_z: Fixed16(0),
            count: 0,
            ceiling: 0,
            dummy: 0,
            colliders: Vec::new(),
        }
    }
}

#[binrw]
#[derive(Debug)]
pub struct Floor {
    pub x: Fixed16,
    pub z: Fixed16,
    pub width: UFixed16,
    pub height: UFixed16,
    pub unknown: u16,
    pub level: u16,
}

#[binrw]
#[derive(Debug)]
struct FloorData {
    num_floors: u16,
    #[br(count = num_floors)]
    floors: Vec<Floor>,
    unknown: u16,
}

/// A parsed representation of an RDT file
///
/// An RDT file defines a room in the game. This parsed RDT representation does not currently
/// contain the entire contents of the RDT. Only collision, floors, and scripts are currently
/// supported.
#[derive(Debug)]
pub struct Rdt {
    collision: Collision,
    floors: Vec<Floor>,
    init_script: Vec<Instruction>,
    exec_script: Vec<Vec<Instruction>>,
}

impl Rdt {
    fn read_script(script_size: u64, reader: &mut Cursor<Vec<u8>>) -> Vec<Instruction> {
        let mut script = Vec::new();

        let mut nesting = 0u32;
        while reader.position() < script_size {
            let inst = match reader.read_le::<Instruction>() {
                Ok(inst) => inst,
                Err(_) => {
                    // we probably just overran the end of the function, so just stop parsing and
                    // return what we have
                    break;
                }
            };
            let is_evt_end = matches!(inst, Instruction::EvtEnd(_));

            if inst.increases_nesting() {
                nesting += 1;
            } else if inst.decreases_nesting() {
                nesting = nesting.saturating_sub(1);
            }

            script.push(inst);
            // the size calculation may not be reliable, so if we see the end-of-function
            // instruction, we'll go ahead and bail
            if is_evt_end && nesting == 0 {
                break;
            }
        }

        script
    }

    pub fn read<T: Read + Seek>(mut f: T) -> Result<Self> {
        let file_size = f.seek(SeekFrom::End(0))?;
        f.seek(SeekFrom::Start(0))?;

        let header: RdtHeader = f.read_le().context("RDT header")?;

        let collision = if header.collision_offset == 0 {
            Collision::default()
        } else {
            f.seek(SeekFrom::Start(header.collision_offset as u64))?;
            f.read_le().context("RDT collision")?
        };

        let floors = if header.floor_offset == 0 {
            Vec::new()
        } else {
            f.seek(SeekFrom::Start(header.floor_offset as u64))?;

            let floor_data: FloorData = f.read_le().context("RDT floor data")?;
            floor_data.floors
        };

        let init_script = if header.init_script_offset == 0 {
            Vec::new()
        } else {
            f.seek(SeekFrom::Start(header.init_script_offset as u64))?;
            let script_size = header.init_script_size();
            let mut buf = vec![0u8; script_size];
            f.read_exact(&mut buf)?;

            Self::read_script(script_size as u64, &mut Cursor::new(buf))
        };

        let exec_script = if header.exec_script_offset == 0 {
            Vec::new()
        } else {
            let exec_script_offset = header.exec_script_offset as u64;
            f.seek(SeekFrom::Start(exec_script_offset))?;

            let mut script_size = header.exec_script_size();
            if exec_script_offset + script_size as u64 > file_size {
                script_size = (file_size - exec_script_offset) as usize;
            }

            if script_size == 0 {
                Vec::new()
            } else {
                let mut buf = vec![0u8; script_size];
                f.read_exact(&mut buf)?;

                let mut reader = Cursor::new(buf);

                let offset: u16 = reader.read_le()?;
                let num_functions = (offset >> 1) as usize;

                let mut offsets = Vec::with_capacity(num_functions + 1);
                offsets.push(offset as u64);
                while offsets.len() < num_functions {
                    let offset = reader.read_le::<u16>()? as u64;
                    offsets.push(offset);
                }
                offsets.push(script_size as u64);

                let mut script = Vec::with_capacity(num_functions);
                for pair in offsets.windows(2) {
                    let offset = pair[0];
                    let next_offset = pair[1];

                    reader.seek(SeekFrom::Start(offset))?;

                    script.push(Self::read_script(next_offset, &mut reader));
                }

                script
            }
        };

        Ok(Self {
            collision,
            floors,
            init_script,
            exec_script,
        })
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.collision.cell_x, self.collision.cell_z)
    }

    pub fn collision(&self) -> &Collision {
        &self.collision
    }

    pub fn floors(&self) -> &[Floor] {
        &self.floors
    }

    pub fn init_script(&self) -> &[Instruction] {
        &self.init_script
    }

    pub fn exec_script(&self) -> impl Iterator<Item = &[Instruction]> {
        self.exec_script.iter().map(|x| x.as_slice())
    }

    pub fn print_scripts(&self) {
        println!("Init script:");
        for instruction in &self.init_script {
            println!("\t{:?}", instruction);
        }

        for (i, function) in self.exec_script.iter().enumerate() {
            println!("\nExec function {}:", i);
            for instruction in function.as_slice() {
                println!("\t{:?}", instruction);
            }
        }
    }
}