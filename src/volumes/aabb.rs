use crate::BoundingVolume;

/// Axis-Aligned Bounding Box
///
/// It is generic over the dimension.
/// Use `Aabb<2>` for 2d, `Aabb<3>` for 3d, etc.
///
/// `Aabb<0>` is valid, though arguably not very useful.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Aabb<const D: usize> {
    min: [f32; D],
    max: [f32; D],
}

impl<const D: usize> Aabb<D> {
    /// Create a bounding box from the min and max points on the volume
    ///
    /// # Panics
    ///
    /// Panic if the a component of max is not greater than min
    #[must_use]
    pub fn from_min_max(min: impl Into<[f32; D]>, max: impl Into<[f32; D]>) -> Self {
        let min: [f32; D] = min.into();
        let max: [f32; D] = max.into();
        for dim in 0..D {
            assert!(min[dim] <= max[dim]);
        }
        Self { min, max }
    }
}

impl<const D: usize> BoundingVolume for Aabb<D> {
    fn merge(mut self, other: Self) -> Self {
        for dim in 0..D {
            if other.min[dim] < self.min[dim] {
                self.min[dim] = other.min[dim];
            }
            if other.max[dim] > self.max[dim] {
                self.max[dim] = other.max[dim];
            }
        }
        self
    }

    fn area(&self) -> f32 {
        (0..D)
            .map(|dim| self.max[dim] - self.min[dim])
            .reduce(|a, b| a * b)
            .unwrap_or_default()
    }

    fn overlaps(&self, other: &Self) -> bool {
        (0..D).all(|dim| self.min[dim] <= other.max[dim] && self.max[dim] >= other.min[dim])
    }
}
