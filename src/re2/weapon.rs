use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::common::*;

/// A rectangular bounding box describing an area where a weapon's hit can land
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HitBounds {
    pub x: Fixed16,
    pub z: Fixed16,
    pub x_size_half: Fixed16,
    pub z_size_quarter: Fixed16,
}

impl HitBounds {
    // most bounds don't have a z offset, so the convenience constructor will omit it for brevity
    pub const fn new(x: i16, x_size_half: i16, z_size_quarter: i16) -> Self {
        Self {
            x: Fixed16(x),
            z: Fixed16(0),
            x_size_half: Fixed16(x_size_half),
            z_size_quarter: Fixed16(z_size_quarter),
        }
    }

    pub const fn zero() -> Self {
        Self {
            x: Fixed16(0),
            z: Fixed16(0),
            x_size_half: Fixed16(0),
            z_size_quarter: Fixed16(0),
        }
    }

    pub const fn has_area(&self) -> bool {
        !self.x_size_half.is_zero() && !self.z_size_quarter.is_zero()
    }
}

impl Default for HitBounds {
    fn default() -> Self {
        Self::zero()
    }
}

/// An identifier for the combination of aim height and target distance of a HitBounds
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum AimZone {
    None = 0x00,
    LowFar = 0x01,
    LowMid = 0x02,
    LowNear = 0x04,
    Mid = 0x08,
    HighNear = 0x10,
    HighMid = 0x20,
    HighFar = 0x40,
    KnifeHigh = 0x80, // don't really know how this works
}

impl Default for AimZone {
    fn default() -> Self {
        Self::None
    }
}

/// The areas and ranges at which a weapon can hit a target
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WeaponRange {
    pub unk00: u8,
    pub aim_zones: [AimZone; 3],
    pub hit_bounds: [HitBounds; 3],
}

impl WeaponRange {
    pub const fn new(aim_zones: [AimZone; 3], hit_bounds: [HitBounds; 3]) -> Self {
        Self {
            unk00: 0,
            aim_zones,
            hit_bounds,
        }
    }

    pub const fn low(bounds0: HitBounds, bounds1: HitBounds, bounds2: HitBounds) -> Self {
        Self {
            unk00: 0,
            aim_zones: [AimZone::LowNear, AimZone::LowMid, AimZone::LowFar],
            hit_bounds: [bounds0, bounds1, bounds2],
        }
    }

    pub const fn mid(bounds0: HitBounds, bounds1: HitBounds, bounds2: HitBounds) -> Self {
        Self {
            unk00: 0,
            aim_zones: [AimZone::Mid, AimZone::Mid, AimZone::Mid],
            hit_bounds: [bounds0, bounds1, bounds2],
        }
    }

    pub const fn high(bounds0: HitBounds, bounds1: HitBounds, bounds2: HitBounds) -> Self {
        Self {
            unk00: 0,
            aim_zones: [AimZone::HighNear, AimZone::HighMid, AimZone::HighFar],
            hit_bounds: [bounds0, bounds1, bounds2],
        }
    }

    pub const fn one(aim_zone: AimZone, bounds: HitBounds) -> Self {
        Self {
            unk00: 0,
            aim_zones: [aim_zone, AimZone::None, AimZone::None],
            hit_bounds: [bounds, HitBounds::zero(), HitBounds::zero()],
        }
    }

    pub const fn none() -> Self {
        Self {
            unk00: 0,
            aim_zones: [AimZone::None; 3],
            hit_bounds: [const { HitBounds::zero() }; 3],
        }
    }

    pub const fn is_empty(&self) -> bool {
        matches!(self.aim_zones, [AimZone::None, AimZone::None, AimZone::None]) || !(self.hit_bounds[0].has_area() || self.hit_bounds[1].has_area() || self.hit_bounds[2].has_area())
    }
}