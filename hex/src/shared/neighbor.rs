use crate::structs::*;
use std::ops::Neg;

pub trait Number: num::Num + num::Signed + Neg<Output = Self> + Ord + Copy {}
impl<T> Number for T where T: num::Num + num::Signed + Neg<Output = Self> + Ord + Copy {}

pub fn get_cube_direction_vectors<T: Number>() -> [Cube<T>; 6] {
    let one = num::one();
    let zero = num::zero();
    return [
        Cube {
            q: one,
            r: zero,
            s: -one,
        },
        Cube {
            q: one,
            r: -one,
            s: zero,
        },
        Cube {
            q: zero,
            r: -one,
            s: one,
        },
        Cube {
            q: -one,
            r: zero,
            s: one,
        },
        Cube {
            q: -one,
            r: one,
            s: zero,
        },
        Cube {
            q: zero,
            r: one,
            s: -one,
        },
    ];
}

pub fn calculate_distance<T: Number>(src: Cube<T>, dst: Cube<T>) -> T {
    let one = num::one::<T>();
    let diff = src - dst;
    return (diff.q.abs() + diff.r.abs() + diff.s.abs()) / (one + one);
}

pub fn get_cubes_within_range<T: Number>(src: Cube<T>, n: T) -> impl Iterator<Item = Cube<T>>
where
    std::ops::Range<T>: Iterator<Item = T>,
{
    let one = num::one();
    (-n..(n + one)).flat_map(move |q| {
        ((-n).max(-q - n)..((n).min(-q + n) + one)).map(move |r| {
            let s = -q - r;
            src + Cube { q, r, s }
        })
    })
}

pub fn get_nth_nearest_cubes<T: Number>(src: Cube<T>, n: T) -> impl Iterator<Item = Cube<T>>
where
    std::ops::Range<T>: Iterator<Item = T>,
{
    get_cubes_within_range(src, n)
        .into_iter()
        .filter(move |cube| calculate_distance(src, *cube) == n)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::flat::coordinate::*;
    use std::collections::HashSet;

    const WIDTH: i32 = 3;
    const HEIGHT: i32 = 3;

    fn filter(cube: &Cube<i32>) -> bool {
        let offset = convert_axial_to_offset(convert_cube_to_axial(*cube));
        return offset.q >= 0 && offset.r >= 0 && offset.q < WIDTH && offset.r < HEIGHT;
    }

    #[test]
    fn test_distance() {
        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 0 }));
        let d = calculate_distance(src, src);
        assert_eq!(d, 0);

        let dst = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        let d = calculate_distance(src, dst);
        assert_eq!(d, 1);

        let dst = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        let d = calculate_distance(src, dst);
        assert_eq!(d, 1);

        let dst = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 0 }));
        let d = calculate_distance(src, dst);
        assert_eq!(d, 2);

        let dst = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        let d = calculate_distance(src, dst);
        assert_eq!(d, 2);

        let dst = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 2 }));
        let d = calculate_distance(src, dst);
        assert_eq!(d, 2);
    }

    #[test]
    fn test_skill_type_point_1() {
        const N: i32 = 1;

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 0 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 0) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 2, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 1) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 5, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 0 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 1) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 2, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 1) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 4, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 2) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 6, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 1 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 2) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 4, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 2 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 2) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 3, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 2 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 2) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 3, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 2 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 1) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 3, "{:?}", result);
    }

    #[test]
    fn test_skill_type_point_2() {
        const N: i32 = 2;

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 0 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 1) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 4, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 2) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 3, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 0 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 2) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 4, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 2) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 4, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 1 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 0) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 2, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 1 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 2) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 4, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 2 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 2) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 4, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 2 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 1) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 3, "{:?}", result);

        let src = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 2 }));
        let result = get_nth_nearest_cubes(src, N);
        let result: HashSet<_> = result.filter(filter).collect();
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 1 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 1) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 0, r: 2 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (0, 2) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 1, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (1, 0) aka {:?}",
            result,
            neighbor
        );
        let neighbor = convert_axial_to_cube(convert_offset_to_axial(Offset { q: 2, r: 0 }));
        assert!(
            result.contains(&neighbor),
            "{:?} doesn't contain (2, 0) aka {:?}",
            result,
            neighbor
        );
        assert_eq!(result.len(), 4, "{:?}", result);
    }
}
