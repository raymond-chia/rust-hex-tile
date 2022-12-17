// this is even-q with y inversed
// in the end, this is more like odd-q

use crate::{shared::round, structs::*};
use std::ops::{BitAnd, Neg};

pub use crate::shared::coordinate::*;

pub fn point_to_offset<I, F>(size: (F, F), point: (F, F)) -> Offset<I>
where
    I: 'static + num::Num + BitAnd<Output = I> + Neg<Output = I> + Copy,
    F: num::Float + num::cast::AsPrimitive<I>,
{
    return axial_to_offset(point_to_axial(size, point));
}

pub fn offset_to_point<I, F>(size: (F, F), offset: Offset<I>) -> (F, F)
where
    I: num::Num + BitAnd<Output = I> + num::cast::AsPrimitive<F>,
    F: 'static + num::Float,
{
    return axial_to_point(size, offset_to_axial(offset));
}

// we use even-q with y inversed
// https://www.redblobgames.com/grids/hexagons/#size-and-spacing
// https://www.redblobgames.com/grids/hexagons/#hex-to-pixel
pub fn point_to_axial<I, F>(size: (F, F), point: (F, F)) -> Axial<I>
where
    I: 'static + num::Num + Neg<Output = I> + Copy,
    F: num::Float + num::cast::AsPrimitive<I>,
{
    let one: F = num::one();
    let q = point.0 / size.0;
    let r = point.1 / size.1;
    let r = r - q / (one + one); // every q contributes half y

    return round::axial_round(Axial { q, r });
}

pub fn axial_to_point<I, F>(size: (F, F), axial: Axial<I>) -> (F, F)
where
    I: num::Num + num::cast::AsPrimitive<F>,
    F: 'static + num::Float,
{
    let q = axial.q;
    let r = axial.r;
    let r = r * (num::one::<I>() + num::one()) + q;
    let q: F = q.as_();
    let r: F = r.as_() / (num::one::<F>() + num::one());
    let q = q * size.0;
    let r = r * size.1;
    return (q, r);
}

// https://www.redblobgames.com/grids/hexagons/#conversions-offset
pub fn axial_to_offset<T>(axial: Axial<T>) -> Offset<T>
where
    T: num::Num + BitAnd<Output = T> + Copy,
{
    let one = num::one();
    let q = axial.q;
    let r = axial.r + (axial.q - (axial.q & one)) / (one + one);
    return Offset { q, r };
}

pub fn offset_to_axial<T>(offset: Offset<T>) -> Axial<T>
where
    T: num::Num + BitAnd<Output = T> + Copy,
{
    let one = num::one();
    let q = offset.q;
    let r = offset.r - (offset.q - (offset.q & one)) / (one + one);
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
            let offset = point_to_offset(TILE_SIZE, case.translation);
            assert_eq!(
                offset, case.exp,
                "input translation: {:?}",
                case.translation
            );

            let axial = offset_to_axial(offset);
            let offset = axial_to_offset(axial);
            assert_eq!(offset, case.exp, "axial: {:?}, offset: {:?}", axial, offset);

            let point = offset_to_point(TILE_SIZE, offset);
            let offset = point_to_offset(TILE_SIZE, point);
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
                translation: (0.0, 30.0),
                exp: Offset { q: 0, r: 1 },
            },
            Case {
                translation: (42.0, 15.0),
                exp: Offset { q: 1, r: 0 },
            },
            Case {
                translation: (42.0, 45.0),
                exp: Offset { q: 1, r: 1 },
            },
            Case {
                translation: (0.0, 60.0),
                exp: Offset { q: 0, r: 2 },
            },
            Case {
                translation: (42.0, 75.0),
                exp: Offset { q: 1, r: 2 },
            },
            Case {
                translation: (84.0, 0.0),
                exp: Offset { q: 2, r: 0 },
            },
            Case {
                translation: (84.0, 30.0),
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
    fn col0() {
        let cases = vec![
            Case {
                translation: (-14.0, 14.0),
                exp: Offset { q: 0, r: 0 },
            },
            Case {
                translation: (-14.0, 16.0),
                exp: Offset { q: 0, r: 1 },
            },
            Case {
                translation: (-14.0, 44.0),
                exp: Offset { q: 0, r: 1 },
            },
            Case {
                translation: (-14.0, 46.0),
                exp: Offset { q: 0, r: 2 },
            },
        ];

        assert_cases(&cases);

        let mut cases = cases;
        cases.iter_mut().for_each(|case| {
            case.translation.0 += 28.0;
        });

        assert_cases(&cases);
    }

    #[test]
    fn col3() {
        let cases = vec![
            Case {
                translation: (112.0, 29.0),
                exp: Offset { q: 3, r: 0 },
            },
            Case {
                translation: (112.0, 31.0),
                exp: Offset { q: 3, r: 1 },
            },
            Case {
                translation: (112.0, 59.0),
                exp: Offset { q: 3, r: 1 },
            },
            Case {
                translation: (112.0, 61.0),
                exp: Offset { q: 3, r: 2 },
            },
        ];

        assert_cases(&cases);

        let mut cases = cases;
        cases.iter_mut().for_each(|case| {
            case.translation.0 += 28.0;
        });

        assert_cases(&cases);
    }

    #[test]
    fn positive_slope() {
        let cases = vec![
            Case {
                translation: (20.0, 7.0),
                exp: Offset { q: 0, r: 0 },
            },
            Case {
                translation: (22.0, 8.0),
                exp: Offset { q: 1, r: 0 },
            },
            Case {
                translation: (62.0, 22.0),
                exp: Offset { q: 1, r: 0 },
            },
            Case {
                translation: (64.0, 23.0),
                exp: Offset { q: 2, r: 1 },
            },
        ];

        assert_cases(&cases);
    }

    #[test]
    fn negative_slope() {
        let cases = vec![
            Case {
                translation: (20.0, 53.0),
                exp: Offset { q: 0, r: 2 },
            },
            Case {
                translation: (22.0, 52.0),
                exp: Offset { q: 1, r: 1 },
            },
            Case {
                translation: (62.0, 38.0),
                exp: Offset { q: 1, r: 1 },
            },
            Case {
                translation: (64.0, 37.0),
                exp: Offset { q: 2, r: 1 },
            },
        ];

        assert_cases(&cases);
    }
}
