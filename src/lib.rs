#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms, missing_docs, clippy::pedantic)]
#![deny(unsafe_code)]
#![no_std]

//! A bounding-volume hierarchy for in-game broad-phase collision detection
//!
//! The core type is [`Bvh`]. It supports insertion and removal as well as iterating over all overlaping pairs.
//!
//! The [`Bvh`] type is generic over the type of bounding volume,
//! so you may choose one in [`volumes`] or implement yourself the [`BoundingVolume`] trait.
//!
//! # Usage example
//!
//! ```
//! use bvh_arena::{Bvh, volumes::Aabb};
//!
//! // The built-in `Aabb` type can be created from anything that implement `Into<[f32; D]>`
//! // So we can use glam's vector types. But you may use another math library or even none.
//! // Obviously, it works the same with `Vec3`.
//! use glam::Vec2;
//!
//! // Create a bounding volume hierarchy.
//! let mut bvh: Bvh<u8, Aabb<2>> = Bvh::default();
//!
//! // Insert bounding volumes
//! bvh.insert(1, Aabb::from_min_max(Vec2::ZERO, Vec2::new(1.0, 1.0)));
//! bvh.insert(2, Aabb::from_min_max(Vec2::new(0.5, 0.5), Vec2::new(2.0, 2.0)));
//! bvh.insert(3, Aabb::from_min_max(Vec2::new(3.0, 2.0), Vec2::new(4.0, 4.0)));
//!
//! // Insertion returns an Id that can be used to remove the bounding volume
//! let id = bvh.insert(4, Aabb::from_min_max(Vec2::new(3.0, 2.0), Vec2::new(4.0, 4.0)));
//! bvh.remove(id);
//!
//! let mut pairs: Vec<(u8, u8)> = Vec::new();
//!
//! // Iteration over overlaping pairs
//! bvh.for_each_overlaping_pair(|data1, data2| pairs.push((*data1, *data2)));
//!
//! assert_eq!(pairs, vec![(1, 2)]);
//! ```
pub use self::bvh::{Bvh, VolumeHandle};

mod bvh;

pub mod volumes;

/// A type that represent a bounding volume, useful for broad collision detection
pub trait BoundingVolume: Copy {
    /// Merge the two bounding volumes into a single one that encompass both of them
    #[must_use]
    fn merge(self, other: Self) -> Self;

    /// Return a scalar value that represent "how big" the volume is.
    #[must_use]
    fn area(&self) -> f32;

    /// Returns true if this overlaps with the other volume
    #[must_use]
    fn overlaps(&self, other: &Self) -> bool;
}
