use std::io::{Read, Seek, SeekFrom};

use anyhow::{bail, Result};
use binrw::{binrw, BinRead, BinReaderExt, VecArgs};

use crate::common::{SSVECTOR, Vec3};

const MIN_MOTION_SIZE: usize = size_of::<SSVECTOR>() * 2;

#[binrw]
#[derive(Debug, Clone, Copy)]
pub struct AnimationHeaderEntry {
    num_frames: u16,
    data_offset: u16,
}

#[binrw]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameFlags(u32);

impl FrameFlags {
    pub const fn index(&self) -> usize {
        (self.0 & 0xfff) as usize
    }

    pub const fn flags(&self) -> u32 {
        self.0 & 0xfffff000
    }
}

#[binrw]
#[derive(Debug, Clone)]
pub struct FramesHeader {
    rel_pos_offset: u16,
    motion_offset: u16,
    count: u16,
    element_size: u16,
}

#[derive(Debug, Clone, BinRead)]
#[br(import { byte_size: usize })]
pub struct FrameMotionData {
    unknown: SSVECTOR,
    speed: SSVECTOR,
    #[br(count = byte_size - MIN_MOTION_SIZE)]
    angles: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct AnimationFrame {
    flags: FrameFlags,
    speed: Vec3,
}

impl AnimationFrame {
    pub const fn index(&self) -> usize {
        self.flags.index()
    }

    pub const fn flags(&self) -> u32 {
        self.flags.flags()
    }

    pub const fn speed(&self) -> Vec3 {
        self.speed
    }
}

#[derive(Debug, Clone)]
pub struct WeaponModel {
    animations: Vec<Vec<AnimationFrame>>,
    mesh_data: Vec<u8>,
    tim_data: Vec<u8>,
}

impl WeaponModel {
    pub fn read<T: Read + Seek>(mut f: T) -> Result<Self> {
        let directory_offset: u32 = f.read_le()?;
        let num_directory_entries: u32 = f.read_le()?;

        if num_directory_entries != 4 {
            bail!("Unexpected number of sections in PLW file");
        }

        f.seek(SeekFrom::Start(directory_offset as u64))?;

        let directory: [u32; 4] = f.read_le()?;

        let animation_steps_section_offset = directory[0] as u64;
        f.seek(SeekFrom::Start(animation_steps_section_offset))?;

        let first_entry: AnimationHeaderEntry = f.read_le()?;
        let num_animations = first_entry.data_offset as usize / size_of::<AnimationHeaderEntry>();

        let mut animation_directory = Vec::with_capacity(num_animations);
        animation_directory.push(first_entry);

        for _ in 1..num_animations {
            animation_directory.push(f.read_le()?);
        }

        let mut animation_flags = Vec::with_capacity(num_animations);
        let mut total_frames = 0usize;
        for entry in &animation_directory {
            f.seek(SeekFrom::Start(animation_steps_section_offset + entry.data_offset as u64))?;
            let frame_flags: Vec<FrameFlags> = f.read_le_args(VecArgs { count: entry.num_frames as usize, inner: () })?;
            total_frames += frame_flags.len();
            animation_flags.push(frame_flags);
        }

        let animation_frames_section_offset = directory[1] as u64;
        f.seek(SeekFrom::Start(animation_frames_section_offset))?;

        let header: FramesHeader = f.read_le()?;
        let element_size = header.element_size as usize;

        let frame_motion = if header.motion_offset == 0 || element_size < MIN_MOTION_SIZE {
            Vec::new()
        } else {
            f.seek(SeekFrom::Start(animation_frames_section_offset + header.motion_offset as u64))?;

            let frame_motion: Vec<FrameMotionData> = f.read_le_args(VecArgs { count: total_frames, inner: FrameMotionDataBinReadArgs { byte_size: element_size } })?;
            frame_motion
        };

        let mesh_data_size = directory[3].saturating_sub(directory[2]) as usize;
        let tim_data_size = directory_offset.saturating_sub(directory[3]) as usize;

        let mesh_data = if mesh_data_size > 0 {
            f.seek(SeekFrom::Start(directory[2] as u64))?;
            let mut mesh_data = vec![0u8; mesh_data_size];
            f.read_exact(&mut mesh_data)?;
            mesh_data
        } else {
            Vec::new()
        };

        let tim_data = if tim_data_size > 0 {
            f.seek(SeekFrom::Start(directory[3] as u64))?;
            let mut tim_data = vec![0u8; tim_data_size];
            f.read_exact(&mut tim_data)?;
            tim_data
        } else {
            Vec::new()
        };

        let mut animations = Vec::with_capacity(num_animations);
        if !frame_motion.is_empty() {
            for frame_flags in animation_flags {
                let mut frames = Vec::with_capacity(frame_flags.len());
                if !frame_flags.is_empty() {
                    for flags in frame_flags {
                        let motion_data = frame_motion[flags.index()].clone();
                        frames.push(AnimationFrame { flags, speed: motion_data.speed.into() });
                    }
                }

                animations.push(frames);
            }
        }

        Ok(Self {
            animations,
            mesh_data,
            tim_data,
        })
    }

    pub const fn animations(&self) -> &[Vec<AnimationFrame>] {
        self.animations.as_slice()
    }

    pub const fn mesh_data(&self) -> &[u8] {
        self.mesh_data.as_slice()
    }

    pub const fn tim_data(&self) -> &[u8] {
        self.tim_data.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;

    #[test]
    fn read_plw() {
        let file = File::open(r"D:\games\BIOHAZARD 2 PC\pl0\PLD\PL0EW11.PLW").unwrap();
        let weapon_model = WeaponModel::read(file).unwrap();
        assert!(weapon_model.animations.len() > 0);
    }

    #[test]
    fn read_plw_with_no_animations() {
        let file = File::open(r"D:\games\BIOHAZARD 2 PC\pl0\PLD\PL0EW10.PLW").unwrap();
        let weapon_model = WeaponModel::read(file).unwrap();
        assert_eq!(weapon_model.animations.len(), 0);
    }
}