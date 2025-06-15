use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use anyhow::{Context, Result};
use binrw::{binrw, BinReaderExt, BinWriterExt};
use enum_map::{Enum, EnumMap, enum_map};

use crate::common::*;
use super::script::Instruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum RdtSection {
    SoundAttributes,
    SoundHeader1,
    SoundBank1,
    SoundHeader2,
    SoundBank2,
    Ota,
    Collision,
    CameraPos,
    CameraZone,
    Light,
    Model,
    Floor,
    Block,
    JpMessage,
    OtherMessage,
    CameraScroll,
    InitScript,
    ExecScript,
    SpriteId,
    SpriteData,
    SpriteTexture,
    ModelTexture,
    Animation,
}

impl RdtSection {
    pub const fn first() -> Self {
        Self::SoundAttributes
    }

    pub const fn next(&self) -> Option<Self> {
        Some(match self {
            Self::SoundAttributes => Self::SoundHeader1,
            Self::SoundHeader1 => Self::SoundBank1,
            Self::SoundBank1 => Self::SoundHeader2,
            Self::SoundHeader2 => Self::SoundBank2,
            Self::SoundBank2 => Self::Ota,
            Self::Ota => Self::Collision,
            Self::Collision => Self::CameraPos,
            Self::CameraPos => Self::CameraZone,
            Self::CameraZone => Self::Light,
            Self::Light => Self::Model,
            Self::Model => Self::Floor,
            Self::Floor => Self::Block,
            Self::Block => Self::JpMessage,
            Self::JpMessage => Self::OtherMessage,
            Self::OtherMessage => Self::CameraScroll,
            Self::CameraScroll => Self::InitScript,
            Self::InitScript => Self::ExecScript,
            Self::ExecScript => Self::SpriteId,
            Self::SpriteId => Self::SpriteData,
            Self::SpriteData => Self::SpriteTexture,
            Self::SpriteTexture => Self::ModelTexture,
            Self::ModelTexture => Self::Animation,
            Self::Animation => return None,
        })
    }
}

#[binrw]
#[derive(Debug, Clone)]
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
    const fn offset(&self, section: RdtSection) -> u32 {
        match section {
            RdtSection::SoundAttributes => self.sound_attr_offset,
            RdtSection::SoundHeader1 => self.sound_header1_offset,
            RdtSection::SoundBank1 => self.sound_bank1_offset,
            RdtSection::SoundHeader2 => self.sound_header2_offset,
            RdtSection::SoundBank2 => self.sound_bank2_offset,
            RdtSection::Ota => self.ota_offset,
            RdtSection::Collision => self.collision_offset,
            RdtSection::CameraPos => self.camera_pos_offset,
            RdtSection::CameraZone => self.camera_zone_offset,
            RdtSection::Light => self.light_offset,
            RdtSection::Model => self.model_offset,
            RdtSection::Floor => self.floor_offset,
            RdtSection::Block => self.block_offset,
            RdtSection::JpMessage => self.jp_message_offset,
            RdtSection::OtherMessage => self.other_message_offset,
            RdtSection::CameraScroll => self.camera_scroll_offset,
            RdtSection::InitScript => self.init_script_offset,
            RdtSection::ExecScript => self.exec_script_offset,
            RdtSection::SpriteId => self.sprite_id_offset,
            RdtSection::SpriteData => self.sprite_data_offset,
            RdtSection::SpriteTexture => self.sprite_texture_offset,
            RdtSection::ModelTexture => self.model_texture_offset,
            RdtSection::Animation => self.animation_offset,
        }
    }

    const fn set_offset(&mut self, section: RdtSection, offset: u32) {
        match section {
            RdtSection::SoundAttributes => self.sound_attr_offset = offset,
            RdtSection::SoundHeader1 => self.sound_header1_offset = offset,
            RdtSection::SoundBank1 => self.sound_bank1_offset = offset,
            RdtSection::SoundHeader2 => self.sound_header2_offset = offset,
            RdtSection::SoundBank2 => self.sound_bank2_offset = offset,
            RdtSection::Ota => self.ota_offset = offset,
            RdtSection::Collision => self.collision_offset = offset,
            RdtSection::CameraPos => self.camera_pos_offset = offset,
            RdtSection::CameraZone => self.camera_zone_offset = offset,
            RdtSection::Light => self.light_offset = offset,
            RdtSection::Model => self.model_offset = offset,
            RdtSection::Floor => self.floor_offset = offset,
            RdtSection::Block => self.block_offset = offset,
            RdtSection::JpMessage => self.jp_message_offset = offset,
            RdtSection::OtherMessage => self.other_message_offset = offset,
            RdtSection::CameraScroll => self.camera_scroll_offset = offset,
            RdtSection::InitScript => self.init_script_offset = offset,
            RdtSection::ExecScript => self.exec_script_offset = offset,
            RdtSection::SpriteId => self.sprite_id_offset = offset,
            RdtSection::SpriteData => self.sprite_data_offset = offset,
            RdtSection::SpriteTexture => self.sprite_texture_offset = offset,
            RdtSection::ModelTexture => self.model_texture_offset = offset,
            RdtSection::Animation => self.animation_offset = offset,
        }
    }

    fn size(&self, in_section: RdtSection) -> Option<u32> {
        let offset = self.offset(in_section);
        if offset == 0 {
            return Some(0);
        }

        let mut size = u32::MAX;
        let mut next_section = Some(RdtSection::first());
        while let Some(section) = next_section {
            next_section = section.next();
            if section == in_section {
                continue;
            }

            let next_offset = self.offset(section);
            if next_offset > offset {
                size = size.min(next_offset - offset);
            }
        }

        (size != u32::MAX).then_some(size)
    }

    fn init_script_size(&self) -> usize {
        self.size(RdtSection::InitScript).unwrap_or(0) as usize
    }

    fn exec_script_size(&self) -> usize {
        self.size(RdtSection::ExecScript).unwrap_or(0) as usize
    }
}

#[derive(Debug, Clone)]
pub struct RawRdt {
    header: RdtHeader,
    sections: EnumMap<RdtSection, Vec<u8>>,
}

macro_rules! read_section {
    ($f:ident, $section:ident, $header:ident) => {
        RawRdt::read_next_section(&mut $f, $section, &$header)?
    }
}

impl RawRdt {
    fn update_header(&mut self) {
        let mut offset = size_of::<RdtHeader>();
        for (section, data) in &self.sections {
            if data.is_empty() {
                self.header.set_offset(section, 0);
            } else {
                self.header.set_offset(section, offset as u32);
                offset += data.len();
            }
        }
    }

    pub fn section(&self, section: RdtSection) -> &[u8] {
        &self.sections[section]
    }

    pub fn replace_section(&mut self, section: RdtSection, data: Vec<u8>) {
        self.sections[section] = data;
        self.update_header();
    }

    pub fn size(&self) -> usize {
        size_of::<RdtHeader>() + self.sections.values().map(Vec::len).sum::<usize>()
    }

    fn read_next_section<T: Read + Seek>(mut f: T, section: RdtSection, header: &RdtHeader) -> Result<Vec<u8>> {
        let offset = header.offset(section) as u64;
        f.seek(SeekFrom::Start(offset))?;
        let buf = match header.size(section) {
            Some(size) => {
                let size = size as usize;
                let mut buf = Vec::with_capacity(size);
                f.read_exact(&mut buf)?;
                buf
            }
            None => {
                let mut buf = Vec::new();
                f.read_to_end(&mut buf)?;
                buf
            }
        };

        Ok(buf)
    }

    pub fn read<T: Read + Seek>(mut f: T) -> Result<Self> {
        use RdtSection::*;

        let header: RdtHeader = f.read_le()?;
        let sections = enum_map! {
            SoundAttributes => read_section!(f, SoundAttributes, header),
            SoundHeader1 => read_section!(f, SoundHeader1, header),
            SoundBank1 => read_section!(f, SoundBank1, header),
            SoundHeader2 => read_section!(f, SoundHeader2, header),
            SoundBank2 => read_section!(f, SoundBank2, header),
            Ota => read_section!(f, Ota, header),
            Collision => read_section!(f, Collision, header),
            CameraPos => read_section!(f, CameraPos, header),
            CameraZone => read_section!(f, CameraZone, header),
            Light => read_section!(f, Light, header),
            Model => read_section!(f, Model, header),
            Floor => read_section!(f, Floor, header),
            Block => read_section!(f, Block, header),
            JpMessage => read_section!(f, JpMessage, header),
            OtherMessage => read_section!(f, OtherMessage, header),
            CameraScroll => read_section!(f, CameraScroll, header),
            InitScript => read_section!(f, InitScript, header),
            ExecScript => read_section!(f, ExecScript, header),
            SpriteId => read_section!(f, SpriteId, header),
            SpriteData => read_section!(f, SpriteData, header),
            SpriteTexture => read_section!(f, SpriteTexture, header),
            ModelTexture => read_section!(f, ModelTexture, header),
            Animation => read_section!(f, Animation, header),
        };

        let mut rdt = Self {
            header,
            sections,
        };
        // go ahead and rewrite the header to eliminate any gaps between the sections so we don't
        // have issues if we immediately write it back out
        rdt.update_header();
        Ok(rdt)
    }

    pub fn write<T: Write + Seek>(&self, mut f: T) -> Result<()> {
        f.write_le(&self.header)?;
        for data in self.sections.values() {
            f.write(data)?;
        }

        Ok(())
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
    pub floors: Vec<Floor>,
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