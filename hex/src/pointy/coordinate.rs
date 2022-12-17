// this is odd-r with y inversed

use crate::{shared::round, structs::*};
use std::ops::{BitAnd, Neg};

pub use crate::shared::coordinate::*;

pub fn convert_point_to_offset<I, F>(size: (F, F), point: (F, F)) -> Offset<I>
where
    I: 'static + num::Num + BitAnd<Output = I> + Neg<Output = I> + Copy,
    F: num::Float + num::cast::AsPrimitive<I>,
{
    return convert_axial_to_offset(convert_point_to_axial(size, point));
}

pub fn convert_offset_to_point<I, F>(size: (F, F), offset: Offset<I>) -> (F, F)
where
    I: num::Num + BitAnd<Output = I> + num::cast::AsPrimitive<F>,
    F: 'static + num::Float,
{
    return convert_axial_to_point(size, convert_offset_to_axial(offset));
}

// convert_point_to_axial uses even-q with y inversed
// https://www.redblobgames.com/grids/hexagons/#size-and-spacing
// https://www.redblobgames.com/grids/hexagons/#hex-to-pixel
pub fn convert_point_to_axial<I, F>(size: (F, F), point: (F, F)) -> Axial<I>
where
    I: 'static + num::Num + BitAnd<Output = I> + Neg<Output = I> + Copy,
    F: num::Float + num::cast::AsPrimitive<I>,
{
    let one: F = num::one();
    let q = point.0 / size.0;
    let r = point.1 / size.1;
    let q = q - r / (one + one); // every q contributes half y

    return round::round_axial(Axial { q, r });
}

pub fn convert_axial_to_point<I, F>(size: (F, F), axial: Axial<I>) -> (F, F)
where
    I: num::Num + num::cast::AsPrimitive<F>,
    F: 'static + num::Float,
{
    let q = axial.q;
    let r = axial.r;
    let q = q * (num::one::<I>() + num::one()) + r;
    let q: F = q.as_() / (num::one::<F>() + num::one());
    let r: F = r.as_();
    let q = q * size.0;
    let r = r * size.1;
    return (q, r);
}

// https://www.redblobgames.com/grids/hexagons/#conversions-offset
pub fn convert_axial_to_offset<T>(axial: Axial<T>) -> Offset<T>
where
    T: num::Num + BitAnd<Output = T> + Copy,
{
    let one = num::one();
    let q = axial.q + (axial.r - (axial.r & one)) / (one + one);
    let r = axial.r;
    return Offset { q, r };
}

pub fn convert_offset_to_axial<T>(offset: Offset<T>) -> Axial<T>
where
    T: num::Num + BitAnd<Output = T> + Copy,
{
    let one = num::one();
    let q = offset.q - (offset.r - (offset.r & one)) / (one + one);
    let r = offset.r;
    return Axial { q, r };
}

#[cfg(test)]
mod test {
    use super::*;
    const TILE_SIZE: (f32, f32) = (42.0, 30.0);

    #[derive(Debug)]
    struct Case {
        translation: (f32, f32),
        exp: Offset<i32>,
    }

    fn assert_cases(cases: &Vec<Case>) {
        cases.iter().for_each(|case| {
            let offset = convert_point_to_offset(TILE_SIZE, case.translation);
            assert_eq!(
                offset, case.exp,
                "input translation: {:?}",
                case.translation
            );

            let axial = convert_offset_to_axial(offset);
            let offset = convert_axial_to_offset(axial);
            assert_eq!(offset, case.exp, "axial: {:?}, offset: {:?}", axial, offset);

            let point = convert_offset_to_point(TILE_SIZE, offset);
            let offset = convert_point_to_offset(TILE_SIZE, point);
            assert_eq!(offset, case.exp)
        });
    }

    #[test]
    fn center() {
        let cases = vec![
            Case {
                translation: (0.0, 0.0),
                exp: Offset { q: 0, r: 0 },
            },
            Case {
                translation: (21.0, 30.0),
                exp: Offset { q: 0, r: 1 },
            },
            Case {
                translation: (42.0, 0.0),
                exp: Offset { q: 1, r: 0 },
            },
            Case {
                translation: (63.0, 30.0),
                exp: Offset { q: 1, r: 1 },
            },
            Case {
                translation: (0.0, 60.0),
                exp: Offset { q: 0, r: 2 },
            },
            Case {
                translation: (42.0, 60.0),
                exp: Offset { q: 1, r: 2 },
            },
            Case {
                translation: (84.0, 0.0),
                exp: Offset { q: 2, r: 0 },
            },
            Case {
                translation: (105.0, 30.0),
                exp: Offset { q: 2, r: 1 },
            },
            Case {
                translation: (84.0, 60.0),
                exp: Offset { q: 2, r: 2 },
            },
        ];

        assert_cases(&cases);
    }

    #[test]
    fn row0() {
        let cases = vec![
            Case {
                translation: (20.0, -7.0),
                exp: Offset { q: 0, r: 0 },
            },
            Case {
                translation: (22.0, -7.0),
                exp: Offset { q: 1, r: 0 },
            },
            Case {
                translation: (62.0, -7.0),
                exp: Offset { q: 1, r: 0 },
            },
            Case {
                translation: (64.0, -7.0),
                exp: Offset { q: 2, r: 0 },
            },
        ];

        assert_cases(&cases);

        let mut cases = cases;
        cases.iter_mut().for_each(|case| {
            case.translation.1 += 14.0;
        });

        assert_cases(&cases);
    }

    #[test]
    fn row3() {
        let cases = vec![
            Case {
                translation: (41.0, 83.0),
                exp: Offset { q: 0, r: 3 },
            },
            Case {
                translation: (63.0, 83.0),
                exp: Offset { q: 1, r: 3 },
            },
            Case {
                translation: (83.0, 83.0),
                exp: Offset { q: 1, r: 3 },
            },
            Case {
                translation: (85.0, 83.0),
                exp: Offset { q: 2, r: 3 },
            },
        ];

        assert_cases(&cases);

        let mut cases = cases;
        cases.iter_mut().for_each(|case| {
            case.translation.1 += 14.0;
        });

        assert_cases(&cases);
    }

    #[test]
    fn positive_slope() {
        let cases = vec![
            Case {
                translation: (20.0, 9.0),
                exp: Offset { q: 0, r: 0 },
            },
            Case {
                translation: (21.0, 11.0),
                exp: Offset { q: 0, r: 1 },
            },
            Case {
                translation: (41.0, 39.0),
                exp: Offset { q: 0, r: 1 },
            },
            Case {
                translation: (42.0, 41.0),
                exp: Offset { q: 1, r: 2 },
            },
        ];

        assert_cases(&cases);
    }

    #[test]
    fn negative_slope() {
        let cases = vec![
            Case {
                translation: (20.0, 50.0),
                exp: Offset { q: 0, r: 2 },
            },
            Case {
                translation: (21.0, 48.0),
                exp: Offset { q: 0, r: 1 },
            },
            Case {
                translation: (41.0, 20.0),
                exp: Offset { q: 0, r: 1 },
            },
            Case {
                translation: (42.0, 18.0),
                exp: Offset { q: 1, r: 0 },
            },
        ];

        assert_cases(&cases);
    }
}
