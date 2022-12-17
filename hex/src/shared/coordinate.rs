use crate::structs::*;
use std::ops::{Add, Neg, Sub};

// https://www.redblobgames.com/grids/hexagons/#conversions-axial
pub fn cube_to_axial<T>(cube: Cube<T>) -> Axial<T> {
    let q = cube.q;
    let r = cube.r;
    return Axial { q, r };
}

// https://www.redblobgames.com/grids/hexagons/#conversions-axial
pub fn axial_to_cube<T>(axial: Axial<T>) -> Cube<T>
where
    T: num::Num + Neg<Output = T> + Copy,
{
    let q = axial.q;
    let r = axial.r;
    let s = -q - r;
    return Cube { q, r, s };
}

impl<T: num::Num> Add for Cube<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cube {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl<T: num::Num> Sub for Cube<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Cube {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
            s: self.s - rhs.s,
        }
    }
}
