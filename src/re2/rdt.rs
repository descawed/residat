use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use anyhow::{anyhow, bail, Context, Result};
use binrw::{binrw, BinReaderExt, BinWriterExt};
use enum_map::{Enum, EnumMap, enum_map};

use crate::common::*;
use super::animation::AnimationSet;
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
    pub const NUM_SECTIONS: usize = 23;

    pub const ALL: [Self; 23] =
        [
            Self::SoundAttributes,
            Self::SoundHeader1,
            Self::SoundBank1,
            Self::SoundHeader2,
            Self::SoundBank2,
            Self::Ota,
            Self::Collision,
            Self::CameraPos,
            Self::CameraZone,
            Self::Light,
            Self::Model,
            Self::Floor,
            Self::Block,
            Self::JpMessage,
            Self::OtherMessage,
            Self::CameraScroll,
            Self::InitScript,
            Self::ExecScript,
            Self::SpriteId,
            Self::SpriteData,
            Self::SpriteTexture,
            Self::ModelTexture,
            Self::Animation,
        ];

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
}

#[derive(Debug, Clone)]
pub struct RawRdt {
    header: RdtHeader,
    sections: EnumMap<RdtSection, Vec<u8>>,
    section_order: Vec<RdtSection>,
}

impl RawRdt {
    pub fn section(&self, section: RdtSection) -> &[u8] {
        &self.sections[section]
    }

    pub fn reader(&self, section: RdtSection) -> Option<Cursor<&[u8]>> {
        (self.header.offset(section) != 0).then(|| Cursor::new(self.section(section)))
    }

    pub fn model_offsets(&self) -> Result<Vec<ModelOffsets>> {
        let raw = self.section(RdtSection::Model);
        let mut reader = Cursor::new(raw);
        let mut offsets = Vec::with_capacity(self.header.o_model as usize);
        for _ in 0..self.header.o_model {
            offsets.push(reader.read_le()?);
        }
        Ok(offsets)
    }

    pub fn set_model_offsets(&mut self, offsets: Vec<ModelOffsets>) -> Result<()> {
        if offsets.len() > u8::MAX as usize {
            bail!("Too many model offsets");
        }

        let mut writer = Cursor::new(Vec::new());
        writer.write_le(&offsets)?;
        let mut buf = writer.into_inner();
        // if there was any extra data in the buffer, leave it there
        let old_data = &self.sections[RdtSection::Model];
        if old_data.len() > buf.len() {
            buf.extend_from_slice(&old_data[buf.len()..]);
        }

        self.replace_section(RdtSection::Model, buf)?;
        self.header.o_model = offsets.len() as u8;
        Ok(())
    }

    fn shift(&mut self, offset: u32, delta: i32) -> Result<()> {
        if delta == 0 {
            return Ok(());
        }

        for &section in &RdtSection::ALL {
            let section_offset = self.header.offset(section);
            if section_offset <= offset {
                continue;
            }

            let new_offset = section_offset.checked_add_signed(delta).unwrap_or(0);
            self.header.set_offset(section, new_offset);
        }

        // need to update model pointers
        let mut model_offsets = self.model_offsets()?;
        for model_offset in &mut model_offsets {
            if model_offset.tim_offset >= offset {
                model_offset.tim_offset = model_offset.tim_offset.checked_add_signed(delta).ok_or_else(|| anyhow!("Overflow while updating model offsets"))?;
            }

            if model_offset.md1_offset >= offset {
                model_offset.md1_offset = model_offset.md1_offset.checked_add_signed(delta).ok_or_else(|| anyhow!("Overflow while updating model offsets"))?;
            }
        }
        self.set_model_offsets(model_offsets)
    }

    pub fn replace_section(&mut self, section: RdtSection, data: Vec<u8>) -> Result<()> {
        if data.is_empty() {
            let original_offset = self.header.offset(section);
            let original_size = self.sections[section].len();
            self.header.set_offset(section, 0);
            self.section_order.retain(|s| *s != section);
            // need to shift things forward
            self.shift(original_offset, -(original_size as i32))?;
        } else if !self.section_order.contains(&section) {
            // add it at the end
            self.section_order.push(section);
            // size will still give us the current size for now because we haven't added the data to
            // the map yet
            self.header.set_offset(section, self.size() as u32);
        } else {
            // need to shift everything that comes after this section
            let offset = self.header.offset(section);
            let original_size = self.sections[section].len();
            self.shift(offset, data.len() as i32 - original_size as i32)?;
        }

        self.sections[section] = data;

        Ok(())
    }

    pub fn size(&self) -> usize {
        size_of::<RdtHeader>() + self.sections.values().map(Vec::len).sum::<usize>()
    }

    pub fn section_size(&self, section: RdtSection) -> usize {
        self.sections[section].len()
    }

    pub fn read<T: Read + Seek>(mut f: T) -> Result<Self> {
        use RdtSection::*;

        let header: RdtHeader = f.read_le()?;

        let mut section_offsets = Vec::with_capacity(RdtSection::NUM_SECTIONS);
        for &section in &RdtSection::ALL {
            let offset = header.offset(section);
            if offset == 0 {
                continue;
            }

            section_offsets.push((section, offset));
        }

        section_offsets.sort_by_key(|(_, offset)| *offset);

        let section_order = section_offsets.into_iter().map(|(section, _)| section).collect::<Vec<_>>();

        let mut sections = enum_map! {
            SoundAttributes => Vec::new(),
            SoundHeader1 => Vec::new(),
            SoundBank1 => Vec::new(),
            SoundHeader2 => Vec::new(),
            SoundBank2 => Vec::new(),
            Ota => Vec::new(),
            Collision => Vec::new(),
            CameraPos => Vec::new(),
            CameraZone => Vec::new(),
            Light => Vec::new(),
            Model => Vec::new(),
            Floor => Vec::new(),
            Block => Vec::new(),
            JpMessage => Vec::new(),
            OtherMessage => Vec::new(),
            CameraScroll => Vec::new(),
            InitScript => Vec::new(),
            ExecScript => Vec::new(),
            SpriteId => Vec::new(),
            SpriteData => Vec::new(),
            SpriteTexture => Vec::new(),
            ModelTexture => Vec::new(),
            Animation => Vec::new(),
        };

        for pair in section_order.windows(2) {
            let this_section = pair[0];
            let this_offset = header.offset(this_section);

            let next_section = pair[1];
            let next_offset = header.offset(next_section);

            let size = (next_offset - this_offset) as usize;
            let mut buf = vec![0u8; size];

            f.seek(SeekFrom::Start(this_offset as u64))?;
            f.read_exact(&mut buf)?;

            sections[this_section] = buf;
        }

        // for the last section, just read to EOF
        if let Some(&section) = section_order.last() {
            let offset = header.offset(section);
            let mut buf = Vec::new();

            f.seek(SeekFrom::Start(offset as u64))?;
            f.read_to_end(&mut buf)?;

            sections[section] = buf;
        }

        Ok(Self {
            header,
            sections,
            section_order,
        })
    }

    pub fn write<T: Write + Seek>(&self, mut f: T) -> Result<()> {
        f.write_le(&self.header)?;

        // account for the possibility of a gap between the header and the first section
        if let Some(&section) = self.section_order.first() {
            let offset = self.header.offset(section) as u64;
            let current_position = f.stream_position()?;
            if offset > current_position {
                f.seek(SeekFrom::Start(offset))?;
            } else if offset < current_position {
                bail!("First section offset is inside the header (section {:?}, offset {})", section, offset);
            }
        }

        for &section in &self.section_order {
            let current_position = f.stream_position()? as u32;
            let expected_offset = self.header.offset(section);
            if current_position != expected_offset {
                bail!("expected section {:?} at offset {} but got {}", section, expected_offset, current_position);
            }

            let data = self.section(section);
            f.write(data)?;
        }

        Ok(())
    }
}

#[binrw]
#[derive(Debug)]
pub struct ModelOffsets {
    pub tim_offset: u32,
    pub md1_offset: u32,
}

#[binrw]
#[derive(Debug)]
pub struct Collider {
    pub x: Fixed16,
    pub z: Fixed16,
    pub w: UFixed16,
    pub h: UFixed16,
    pub collision_mask: u16,
    pub quadrant_mask: u16,
    pub floor: u32,
}

impl Collider {
    pub const fn collision_mask(&self) -> u16 {
        self.collision_mask & 0xfff0
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
    raw: RawRdt,
    collision: Collision,
    floors: Vec<Floor>,
    init_script: Vec<Vec<Instruction>>,
    exec_script: Vec<Vec<Instruction>>,
    animation_sets: Vec<AnimationSet>,
}

impl Rdt {
    fn read_function(script_size: u64, reader: &mut Cursor<Vec<u8>>) -> Vec<Instruction> {
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

    fn read_script(raw: &RawRdt, section: RdtSection) -> Result<Vec<Vec<Instruction>>> {
        Ok(if let Some(mut reader) = raw.reader(section) {
            let script_size = raw.section_size(section);

            if script_size == 0 {
                Vec::new()
            } else {
                let mut buf = vec![0u8; script_size];
                reader.read_exact(&mut buf)?;

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

                    script.push(Self::read_function(next_offset, &mut reader));
                }

                script
            }
        } else {
            Vec::new()
        })
    }

    pub fn read<T: Read + Seek>(f: T) -> Result<Self> {
        let raw = RawRdt::read(f)?;

        let collision = if let Some(mut collision_reader) = raw.reader(RdtSection::Collision) {
            collision_reader.read_le().context("RDT collision")?
        } else {
            Collision::default()
        };

        let floors = if let Some(mut floor_reader) = raw.reader(RdtSection::Floor) {
            let floor_data: FloorData = floor_reader.read_le().context("RDT floor data")?;
            floor_data.floors
        } else {
            Vec::new()
        };

        let init_script = Self::read_script(&raw, RdtSection::InitScript)?;

        let exec_script = Self::read_script(&raw, RdtSection::ExecScript)?;

        let animation_sets = if let Some(animation_reader) = raw.reader(RdtSection::Animation) {
            AnimationSet::read_rdt(animation_reader).context("RDT animation")?
        } else {
            Vec::new()
        };

        Ok(Self {
            raw,
            collision,
            floors,
            init_script,
            exec_script,
            animation_sets,
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

    pub fn init_script(&self) -> impl Iterator<Item = &[Instruction]> {
        self.init_script.iter().map(|x| x.as_slice())
    }

    pub fn exec_script(&self) -> impl Iterator<Item = &[Instruction]> {
        self.exec_script.iter().map(|x| x.as_slice())
    }

    pub fn animation_sets(&self) -> &[AnimationSet] {
        &self.animation_sets
    }

    pub fn raw(&self, section: RdtSection) -> &[u8] {
        self.raw.section(section)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(size_of::<Collider>(), 0x10);
    }
}