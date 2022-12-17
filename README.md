# rust-hex-tile
some hexagonal related functions based on https://www.redblobgames.com/grids/hexagons/

## Example
```rust
use hex::{self, pointy::coordinate, pointy::neighbor};

fn main() {
    // to get tiles next to src
    let src = hex::Offset { q: 0, r: 0 };
    let src: hex::Axial<i32> = coordinate::convert_offset_to_axial(src);
    let src: hex::Cube<i32> = coordinate::convert_axial_to_cube(src);
    let tiles = neighbor::get_nth_nearest_cubes(src, 1)
        .into_iter()
        .map(|cube| coordinate::convert_cube_to_axial(cube))
        .map(|axial| coordinate::convert_axial_to_offset(axial));
}
```
