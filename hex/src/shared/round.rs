use crate::{shared::coordinate::*, structs::*};
use std::ops::Neg;

// https://www.redblobgames.com/grids/hexagons/#rounding
pub fn cube_round<I, F>(frac: Cube<F>) -> Cube<I>
where
    I: 'static + num::Num + Neg<Output = I> + Copy,
    F: num::Float + num::cast::AsPrimitive<I>,
{
    let q = frac.q.round();
    let r = frac.r.round();
    let s = frac.s.round();

    let q_diff = (q - frac.q).abs();
    let r_diff = (r - frac.r).abs();
    let s_diff = (s - frac.s).abs();

    let q = q.as_();
    let r = r.as_();
    let s = s.as_();

    if q_diff > r_diff && q_diff > s_diff {
        let q = -r - s;
        return Cube { q, r, s };
    }
    if r_diff > s_diff {
        let r = -q - s;
        return Cube { q, r, s };
    }
    let s = -q - r;
    return Cube { q, r, s };
}

pub fn axial_round<I, F>(axial: Axial<F>) -> Axial<I>
where
    I: 'static + num::Num + Neg<Output = I> + Copy,
    F: num::Float + num::cast::AsPrimitive<I>,
{
    return cube_to_axial(cube_round(axial_to_cube(axial)));
}
