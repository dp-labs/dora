use core::ops::{Add, AddAssign, Deref};
use wasmer::{FromToNativeWasmType, WasmPtr};

/// Represents a pointer to a Guest WASM's memory.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct GuestPtr(pub u32);

impl Add<u32> for GuestPtr {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u32> for GuestPtr {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

impl From<GuestPtr> for u32 {
    fn from(value: GuestPtr) -> Self {
        value.0
    }
}

impl From<GuestPtr> for u64 {
    fn from(value: GuestPtr) -> Self {
        value.0.into()
    }
}

impl Deref for GuestPtr {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GuestPtr {
    pub fn to_u64(self) -> u64 {
        self.into()
    }
}

unsafe impl FromToNativeWasmType for GuestPtr {
    type Native = i32;

    fn from_native(native: i32) -> Self {
        Self(u32::from_native(native))
    }

    fn to_native(self) -> i32 {
        self.0.to_native()
    }
}

impl<T> From<GuestPtr> for WasmPtr<T> {
    fn from(value: GuestPtr) -> Self {
        WasmPtr::new(value.0)
    }
}
