//! Collection of bounding volumes that can be used with the BVH
//!
//! Currently, the only option is an axis-aligned-bounding-box ([`Aabb`])
//!
//! One may implement they own bounding volume by implementing the [`BoundingVolume`](crate::BoundingVolume) trait.

pub use aabb::Aabb;

mod aabb;
