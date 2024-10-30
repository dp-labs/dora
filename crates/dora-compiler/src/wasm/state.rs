use crate::state::ExtraInfo;
use std::ops::{BitAnd, BitOr, BitOrAssign};

impl ExtraInfo {
    // This value is required to be arithmetic 32-bit NaN (or 32x4) by the WAsm
    // machine, but which might not be in the LLVM value. The conversion to
    // arithmetic NaN is pending. It is required for correctness.
    //
    // When applied to a 64-bit value, this flag has no meaning and must be
    // ignored. It may be set in such cases to allow for common handling of
    // 32 and 64-bit operations.
    pub const fn pending_f32_nan() -> ExtraInfo {
        ExtraInfo { state: 1 }
    }

    // This value is required to be arithmetic 64-bit NaN (or 64x2) by the WAsm
    // machine, but which might not be in the LLVM value. The conversion to
    // arithmetic NaN is pending. It is required for correctness.
    //
    // When applied to a 32-bit value, this flag has no meaning and must be
    // ignored. It may be set in such cases to allow for common handling of
    // 32 and 64-bit operations.
    pub const fn pending_f64_nan() -> ExtraInfo {
        ExtraInfo { state: 2 }
    }

    // This value either does not contain a 32-bit NaN, or it contains an
    // arithmetic NaN. In SIMD, applies to all 4 lanes.
    pub const fn arithmetic_f32() -> ExtraInfo {
        ExtraInfo { state: 4 }
    }

    // This value either does not contain a 64-bit NaN, or it contains an
    // arithmetic NaN. In SIMD, applies to both lanes.
    pub const fn arithmetic_f64() -> ExtraInfo {
        ExtraInfo { state: 8 }
    }

    pub const fn has_pending_f32_nan(&self) -> bool {
        self.state & ExtraInfo::pending_f32_nan().state != 0
    }
    pub const fn has_pending_f64_nan(&self) -> bool {
        self.state & ExtraInfo::pending_f64_nan().state != 0
    }
    pub const fn is_arithmetic_f32(&self) -> bool {
        self.state & ExtraInfo::arithmetic_f32().state != 0
    }
    pub const fn is_arithmetic_f64(&self) -> bool {
        self.state & ExtraInfo::arithmetic_f64().state != 0
    }

    pub const fn strip_pending(&self) -> ExtraInfo {
        ExtraInfo {
            state: self.state
                & !(ExtraInfo::pending_f32_nan().state | ExtraInfo::pending_f64_nan().state),
        }
    }
}

// Union two ExtraInfos.
impl BitOr for ExtraInfo {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        debug_assert!(!(self.has_pending_f32_nan() && other.has_pending_f64_nan()));
        debug_assert!(!(self.has_pending_f64_nan() && other.has_pending_f32_nan()));
        ExtraInfo {
            state: if self.is_arithmetic_f32() || other.is_arithmetic_f32() {
                ExtraInfo::arithmetic_f32().state
            } else if self.has_pending_f32_nan() || other.has_pending_f32_nan() {
                ExtraInfo::pending_f32_nan().state
            } else {
                0
            } + if self.is_arithmetic_f64() || other.is_arithmetic_f64() {
                ExtraInfo::arithmetic_f64().state
            } else if self.has_pending_f64_nan() || other.has_pending_f64_nan() {
                ExtraInfo::pending_f64_nan().state
            } else {
                0
            },
        }
    }
}
impl BitOrAssign for ExtraInfo {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

// Intersection for ExtraInfo.
impl BitAnd for ExtraInfo {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        // Pending canonicalizations are not safe to discard, or even reorder.
        debug_assert!(
            self.has_pending_f32_nan() == other.has_pending_f32_nan()
                || self.is_arithmetic_f32()
                || other.is_arithmetic_f32()
        );
        debug_assert!(
            self.has_pending_f64_nan() == other.has_pending_f64_nan()
                || self.is_arithmetic_f64()
                || other.is_arithmetic_f64()
        );
        let info = match (
            self.is_arithmetic_f32() && other.is_arithmetic_f32(),
            self.is_arithmetic_f64() && other.is_arithmetic_f64(),
        ) {
            (false, false) => Default::default(),
            (true, false) => ExtraInfo::arithmetic_f32(),
            (false, true) => ExtraInfo::arithmetic_f64(),
            (true, true) => ExtraInfo::arithmetic_f32() | ExtraInfo::arithmetic_f64(),
        };
        match (self.has_pending_f32_nan(), self.has_pending_f64_nan()) {
            (false, false) => info,
            (true, false) => info | ExtraInfo::pending_f32_nan(),
            (false, true) => info | ExtraInfo::pending_f64_nan(),
            (true, true) => unreachable!("Can't form ExtraInfo with two pending canonicalizations"),
        }
    }
}
