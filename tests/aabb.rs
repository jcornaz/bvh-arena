#![no_std]

use bvh_arena::volumes::Aabb;
use bvh_arena::BoundingVolume;

#[test]
fn overlaps_self() {
    let aabb = Aabb::from_min_max([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
    assert!(aabb.overlaps(&aabb))
}

#[test]
fn overlaps_other() {
    let aabb1 = Aabb::from_min_max([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let aabb2 = Aabb::from_min_max([0.5, -0.5, -0.5], [1.5, 0.5, 1.5]);
    assert!(aabb1.overlaps(&aabb2))
}

#[test]
fn does_not_overlap() {
    let aabb1 = Aabb::from_min_max([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let aabb2 = Aabb::from_min_max([0.5, -0.5, -0.5], [1.5, 0.5, -0.1]);
    assert!(!aabb1.overlaps(&aabb2))
}

#[test]
fn merge_self() {
    let aabb = Aabb::from_min_max([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
    let merged = aabb.merge(aabb);
    assert_eq!(aabb, merged);
}

#[test]
fn merge_other() {
    let aabb1 = Aabb::from_min_max([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let aabb2 = Aabb::from_min_max([0.5, -0.5, -0.5], [1.5, 0.5, 1.5]);
    let merged = aabb1.merge(aabb2);
    assert_eq!(
        merged,
        Aabb::from_min_max([0.0, -0.5, -0.5], [1.5, 1.0, 1.5])
    );
}

#[test]
fn area() {
    assert_eq!(
        Aabb::from_min_max([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]).area(),
        1.0
    );
    assert_eq!(
        Aabb::from_min_max([-1.0, 0.0, 0.0], [1.0, 1.0, 1.0]).area(),
        2.0
    );
    assert_eq!(
        Aabb::from_min_max([-1.0, 0.0, 0.0], [1.0, 2.0, 1.0]).area(),
        4.0
    );
    assert_eq!(
        Aabb::from_min_max([-1.0, 0.0, -2.0], [1.0, 2.0, 1.0]).area(),
        12.0
    );
}
