#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cube<T> {
    pub q: T,
    pub r: T,
    pub s: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Axial<T> {
    pub q: T,
    pub r: T,
}

#[derive(Debug, PartialEq)]
pub struct Offset<T> {
    pub q: T,
    pub r: T,
}
