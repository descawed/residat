#![allow(non_camel_case_types)]

use binrw::binrw;

mod math;
pub use math::*;

/// A wrapper around a 32-bit pointer in the game's address space which is 32 bits on all architectures
#[cfg(target_pointer_width = "32")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Ptr32<T>(*const T);

#[cfg(target_pointer_width = "32")]
impl<T> Ptr32<T> {
    /// Get the wrapped pointer
    ///
    /// On 32-bit architectures, this returns the actual pointer (which may be null). On 64-bit
    /// architectures, this always returns null, as the pointer is necessarily not valid in this
    /// address space.
    pub const fn ptr(&self) -> *const T {
        self.0
    }

    /// Get the bits of the wrapped pointer as a 32-bit unsigned integer
    pub fn as_int(&self) -> u32 {
        self.0 as usize as u32
    }
}

/// A wrapper around a 32-bit pointer in the game's address space which is 32 bits on all architectures
#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ptr32<T> {
    value: u32,
    phantom: std::marker::PhantomData<*const T>,
}

#[cfg(target_pointer_width = "64")]
impl<T> Ptr32<T> {
    /// Get the wrapped pointer
    ///
    /// On 32-bit architectures, this returns the actual pointer (which may be null). On 64-bit
    /// architectures, this always returns null, as the pointer is necessarily not valid in this
    /// address space.
    pub const fn ptr(&self) -> *const T {
        std::ptr::null()
    }

    /// Get the bits of the wrapped pointer as a 32-bit unsigned integer
    pub fn as_int(&self) -> u32 {
        self.value
    }
}

/// A 3D vector with 16-bit components, padded to 8 bytes
#[repr(C)]
#[binrw]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SVECTOR {
    pub vx: Fixed16,
    pub vy: Fixed16,
    pub vz: Fixed16,
    pub pad: Fixed16,
}

/// A 3D vector with 16-bit components and no padding
#[repr(C)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SSVECTOR {
    pub vx: Fixed16,
    pub vy: Fixed16,
    pub vz: Fixed16,
}

/// A 3D vector with 32-bit components
#[repr(C)]
#[binrw]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VECTOR {
    pub x: Fixed32,
    pub y: Fixed32,
    pub z: Fixed32,
}

/// A color vector
#[repr(C)]
#[binrw]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CVECTOR {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub cd: u8,
}

/// A 3x3 transformation matrix plus a translation vector
#[repr(C)]
#[binrw]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MATRIX {
    pub m: [Fixed16; 9],
    pub pad: u16,
    pub t: VECTOR,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(size_of::<SVECTOR>(), 8);
        assert_eq!(size_of::<VECTOR>(), 12);
        assert_eq!(size_of::<MATRIX>(), 32);
    }
}