pub mod prelude {
    use super::Vec2;
    use super::Vec2Index::*;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Vec2<T = f32> {
    pub x: T,
    pub y: T,
}

pub enum Vec2Index {
    X,
    Y,
}

impl core::ops::Index<Vec2Index> for Vec2 {
    type Output = f32;

    fn index(&self, index: Vec2Index) -> &Self::Output {
        match index {
            Vec2Index::X => &self.x,
            Vec2Index::Y => &self.y,
        }
    }
}

impl core::ops::IndexMut<Vec2Index> for Vec2 {
    fn index_mut(&mut self, index: Vec2Index) -> &mut Self::Output {
        match index {
            Vec2Index::X => &mut self.x,
            Vec2Index::Y => &mut self.y,
        }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y }
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(vec: Vec2<T>) -> Self {
        (vec.x, vec.y)
    }
}

impl<T> From<Vec2<T>> for [T; 2] {
    fn from(vec: Vec2<T>) -> Self {
        [vec.x, vec.y]
    }
}
