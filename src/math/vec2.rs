use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /*================== CONSTANTS ==================*/

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };

    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };

    pub const UP: Self = Self { x: 0.0, y: 1.0 };

    pub const DOWN: Self = Self { x: 0.0, y: -1.0 };

    /*================== CONSTRUCTORS ==================*/

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /*================== VECTOR OPERATIONS ==================*/

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let length = self.length();

        if length > 0.0 {
            self / length
        } else {
            Self::ZERO
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn mul_vec(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    /*================== CONVERSION ==================*/

    pub const fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

/*==============================================================*/
/*                             Add                              */
/*==============================================================*/

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

/*==============================================================*/
/*                             Sub                              */
/*==============================================================*/

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

/*==============================================================*/
/*                             Mul                              */
/*==============================================================*/

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

/*==============================================================*/
/*                             Div                              */
/*==============================================================*/

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

/*==============================================================*/
/*                         Conversions                          */
/*==============================================================*/

impl From<Vec2> for [f32; 2] {
    fn from(value: Vec2) -> Self {
        [value.x, value.y]
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(value: [f32; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

/*==============================================================*/
/*                         Negation                             */
/*==============================================================*/

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
