use core::{
    cmp::Ordering,
    ops::{Add, Mul, Neg},
};

/// Never `NaN`, but can be any other [`f64`].
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct NotNaN(f64);

impl NotNaN {
    pub const INFINITY: Self = Self(f64::INFINITY);
}

/// Never `NaN` and always has a positive sign. May be positive infinity.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PositiveNotNaN(f64);

/// Never `NaN` and is always larger than (and not equal to) 0. May be positive
/// infinity.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PositiveNotNaNNotZero(f64);

impl From<PositiveNotNaNNotZero> for PositiveNotNaN {
    fn from(value: PositiveNotNaNNotZero) -> Self {
        Self(value.0)
    }
}

impl From<PositiveNotNaN> for NotNaN {
    fn from(value: PositiveNotNaN) -> Self {
        Self(value.0)
    }
}

impl PositiveNotNaN {
    pub const ZERO: Self = Self(0.0);
    pub const ONE: Self = Self(1.0);
}

impl PositiveNotNaNNotZero {
    pub const TWO: Self = Self(2.0);
}

// The following cases result in NaN:
//  - `NaN * x`: We ensure neither input is NaN.
//  - `0 * Infinity`: We handle this by ensuring at least one side is not 0.
//
// IEEE Std 754-2008 7.2:
// > a) any general-computational or signaling-computational operation on a
// > signaling NaN (see 6.2), except for some conversions (see 5.12)
// > b) multiplication: multiplication(0, ∞) or multiplication(∞, 0)
impl Mul<PositiveNotNaNNotZero> for PositiveNotNaN {
    type Output = Self;

    fn mul(self, rhs: PositiveNotNaNNotZero) -> Self {
        Self(self.0 * rhs.0)
    }
}

// The following cases result in NaN:
//  - `NaN + x`: We ensure neither input is NaN.
//  - `Infinity + -Infinity`: We handle this by ensuring the inputs are
//    positive.
//
// IEEE Std 754-2008 7.2:
// > a) any general-computational or signaling-computational operation on a
// > signaling NaN (see 6.2), except for some conversions (see 5.12)
// > b) addition or subtraction or fusedMultiplyAdd: magnitude subtraction of infinities, such as:
// > addition(+∞, −∞)
impl Add for PositiveNotNaN {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

// Negating a non-NaN value results in a non-NaN value.
impl Neg for NotNaN {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl PartialEq for NotNaN {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for NotNaN {}

impl PartialOrd for NotNaN {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NotNaN {
    fn cmp(&self, other: &Self) -> Ordering {
        // SAFETY: The value is guaranteed to not be NaN. See above.
        unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() }
    }
}
