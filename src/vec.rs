use std::ops::{Add, Sub, Mul, Index};

/// A 2D vector with signed integers as coordinates.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Vec2 {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
}

impl Vec2 {
    /// Create a new Vec2 from the coordintes.
    pub const fn new(x: i32, y: i32) -> Vec2 {
        Vec2 {
            x: x,
            y: y,
        }
    }

    /// The squared distance to another vector.
    pub fn dist_sq(self, other: Vec2) -> u32 {
        (self - other).norm()
    }

    /// The euclidean norm (the sum of the squared components).
    pub fn norm(self) -> u32 {
        (self.x * self.x + self.y * self.y) as u32
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Index by a Vec2.
pub trait GetPos {
    /// The output of `get_pos()`.
    type Output: Copy;

    /// Get element by position (Vec2)
    fn get_pos(&self, vec: Vec2) -> Self::Output;
}

impl<T: Copy, I: Index<usize, Output = J>, J: Index<usize, Output = T>> GetPos for I {
    type Output = T;

    fn get_pos(&self, vec: Vec2) -> T {
        self[vec.y as usize][vec.x as usize]
    }
}
