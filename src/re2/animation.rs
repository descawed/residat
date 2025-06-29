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
pub struct AnimationSet {
    animations: Vec<Vec<AnimationFrame>>,
    character_mask: u32,
}

impl AnimationSet {
    pub const fn new(animations: Vec<Vec<AnimationFrame>>, character_mask: u32) -> Self {
        Self { animations, character_mask }
    }

    pub const fn from_model(animations: Vec<Vec<AnimationFrame>>) -> Self {
        Self::new(animations, 0xffffffff)
    }

    pub const fn animations(&self) -> &[Vec<AnimationFrame>] {
        self.animations.as_slice()
    }

    pub const fn character_mask(&self) -> u32 {
        self.character_mask
    }

    fn read_steps<T: Read + Seek>(mut f: T, start: u64) -> Result<(Vec<Vec<FrameFlags>>, usize)> {
        f.seek(SeekFrom::Start(start))?;

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
            f.seek(SeekFrom::Start(start + entry.data_offset as u64))?;
            let frame_flags: Vec<FrameFlags> = f.read_le_args(VecArgs { count: entry.num_frames as usize, inner: () })?;
            total_frames = total_frames.max(frame_flags.iter().map(FrameFlags::index).max().unwrap_or(0) + 1);
            animation_flags.push(frame_flags);
        }

        Ok((animation_flags, total_frames))
    }

    fn read_frames<T: Read + Seek>(mut f: T, start: u64, total_frames: usize) -> Result<Vec<FrameMotionData>> {
        f.seek(SeekFrom::Start(start))?;

        let header: FramesHeader = f.read_le()?;
        let element_size = header.element_size as usize;

        Ok(if header.motion_offset == 0 || element_size < MIN_MOTION_SIZE {
            Vec::new()
        } else {
            f.seek(SeekFrom::Start(start + header.motion_offset as u64))?;

            let frame_motion: Vec<FrameMotionData> = f.read_le_args(VecArgs { count: total_frames, inner: FrameMotionDataBinReadArgs { byte_size: element_size } })?;
            frame_motion
        })
    }

    fn combine_data(animation_flags: Vec<Vec<FrameFlags>>, frame_motion: Vec<FrameMotionData>) -> Vec<Vec<AnimationFrame>> {
        let mut animations = Vec::with_capacity(animation_flags.len());
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

        animations
    }

    pub fn read_plw<T: Read + Seek>(mut f: T) -> Result<Self> {
        let directory_offset: u32 = f.read_le()?;
        let num_directory_entries: u32 = f.read_le()?;

        if num_directory_entries < 2 {
            bail!("Not enough sections to read animation data");
        }

        f.seek(SeekFrom::Start(directory_offset as u64))?;

        let directory: Vec<u32> = f.read_le_args(VecArgs { count: num_directory_entries as usize, inner: () })?;

        let animation_steps_section_offset = directory[0] as u64;
        let (animation_flags, total_frames) = Self::read_steps(&mut f, animation_steps_section_offset)?;

        let animation_frames_section_offset = directory[1] as u64;
        let frame_motion = Self::read_frames(&mut f, animation_frames_section_offset, total_frames)?;

        Ok(Self::from_model(Self::combine_data(animation_flags, frame_motion)))
    }

    pub fn read_rdt<T: Read + Seek>(mut f: T) -> Result<Vec<Self>> {
        let start = f.stream_position()?;

        let directory_offset: u32 = f.read_le()?;
        let num_directory_entries: u32 = f.read_le()?;

        f.seek(SeekFrom::Start(start + directory_offset as u64))?;

        let directory: Vec<(u32, u32)> = f.read_le_args(VecArgs { count: num_directory_entries as usize, inner: () })?;

        let mut animation_sets = Vec::with_capacity(num_directory_entries as usize);
        for (frames_offset, steps_offset) in directory {
            let animation_steps_section_offset = steps_offset as u64;
            let (animation_flags, total_frames) = Self::read_steps(&mut f, start + animation_steps_section_offset)?;

            let animation_frames_section_offset = frames_offset as u64;
            f.seek(SeekFrom::Start(start + animation_frames_section_offset))?;
            let character_mask: u32 = f.read_le()?;
            let frame_motion = Self::read_frames(&mut f, start + animation_frames_section_offset + 4, total_frames)?;

            animation_sets.push(Self::new(Self::combine_data(animation_flags, frame_motion), character_mask));
        }

        Ok(animation_sets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;

    #[test]
    fn read_plw() {
        let file = File::open(r"D:\games\BIOHAZARD 2 PC\pl0\PLD\PL0EW11.PLW").unwrap();
        let set = AnimationSet::read_plw(file).unwrap();
        assert!(set.animations.len() > 0);
    }

    #[test]
    fn read_plw_with_no_animations() {
        let file = File::open(r"D:\games\BIOHAZARD 2 PC\pl0\PLD\PL0EW10.PLW").unwrap();
        let set = AnimationSet::read_plw(file).unwrap();
        assert_eq!(set.animations.len(), 0);
    }

    #[test]
    fn read_rdt() {
        let mut file = File::open(r"D:\games\BIOHAZARD 2 PC\pl0\Rdt\ROOM10C0.RDT").unwrap();
        file.seek(SeekFrom::Start(0x60)).unwrap();
        let animation_offset: u32 = file.read_le().unwrap();
        file.seek(SeekFrom::Start(animation_offset as u64)).unwrap();

        let sets = AnimationSet::read_rdt(file).unwrap();
        assert!(sets.len() > 0);
        assert_ne!(sets[0].character_mask & 1, 0);
        assert!(sets[0].animations.len() > 0);
    }
}