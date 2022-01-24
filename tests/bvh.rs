#![no_std]

extern crate alloc;

use alloc::collections::BTreeSet;

use glam::Vec2;

use bvh_arena::{volumes::Aabb, Bvh};

fn bvh() -> Bvh<u8, Aabb<1>> {
    Bvh::default()
}

fn range(min: f32, max: f32) -> Aabb<1> {
    Aabb::from_min_max([min], [max])
}

#[test]
fn empty_has_no_overlaps() {
    bvh().for_each_overlaping_pair(|_, _| panic!("A pair was found"));
}

#[test]
fn colliding_pair() {
    let mut bvh = bvh();
    bvh.insert(0, range(0.0, 2.0));
    bvh.insert(1, range(1.0, 3.0));
    let mut count = 0;
    bvh.for_each_overlaping_pair(|a, b| {
        count += 1;
        assert!((*a, *b) == (0, 1) || (*a, *b) == (1, 0));
    });
    assert_eq!(count, 1);
}

#[test]
fn colliding_with() {
    let mut bvh = bvh();
    bvh.insert(0, range(0.0, 2.0));
    bvh.insert(1, range(3.0, 4.0));
    let mut count = 0;
    bvh.for_each_overlaps(&range(1.0, 2.5), |found| {
        count += 1;
        assert_eq!(*found, 0);
    });
    assert_eq!(count, 1);
}

#[test]
fn after_remove() {
    let mut bvh = bvh();
    let id = bvh.insert(0, range(0.0, 2.0));
    bvh.insert(1, range(1.0, 3.0));
    bvh.remove(id);
    bvh.for_each_overlaping_pair(|_, _| {
        panic!("A pair was found");
    });
}

#[test]
fn non_colliding_pair() {
    let mut bvh = bvh();
    bvh.insert(0, range(0.0, 2.0));
    bvh.insert(1, range(2.5, 3.0));
    bvh.for_each_overlaping_pair(|_, _| panic!("A pair was found"));
}

/// Test the following scenario:
///  
///                ┌───┐
///                │ 1 │
/// ┌────┐         │   │
/// │ 0  │         └───┘
/// │    │
/// └────┘
///           ┌──────────┐
///     ┌─────┼──┐  3   ┌┼──┐
///     │     └──┼──────┼┘  │
///     │   2    │      │ 5 │
///     │       ┌┼─────┐│   │
///     │       ││  4  │└───┘
///     └───────┼┘     │
///             └──────┘
///
/// Expected pairs are: 2-3, 2-4, 3-5
#[test]
fn complex_bvh() {
    let mut bvh: Bvh<u8, Aabb<2>> = Bvh::default();
    bvh.insert(
        0,
        Aabb::from_min_max(Vec2::new(0.0, 8.0), Vec2::new(5.0, 11.0)),
    );
    bvh.insert(
        1,
        Aabb::from_min_max(Vec2::new(15.0, 10.0), Vec2::new(19.0, 13.0)),
    );
    bvh.insert(
        2,
        Aabb::from_min_max(Vec2::new(4.0, 1.0), Vec2::new(13.0, 6.0)),
    );
    bvh.insert(
        3,
        Aabb::from_min_max(Vec2::new(10.0, 5.0), Vec2::new(21.0, 7.0)),
    );
    bvh.insert(
        4,
        Aabb::from_min_max(Vec2::new(12.0, 0.0), Vec2::new(19.0, 3.0)),
    );
    bvh.insert(
        5,
        Aabb::from_min_max(Vec2::new(20.0, 2.0), Vec2::new(24.0, 6.0)),
    );

    let id = bvh.insert(
        6,
        Aabb::from_min_max(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0)),
    );
    bvh.remove(id);

    let mut expected: BTreeSet<(u8, u8)> = BTreeSet::new();
    expected.insert((2, 3));
    expected.insert((2, 4));
    expected.insert((3, 5));

    let mut actual: BTreeSet<(u8, u8)> = BTreeSet::new();
    bvh.for_each_overlaping_pair(|a, b| {
        if a < b {
            actual.insert((*a, *b));
        } else {
            actual.insert((*b, *a));
        }
    });

    assert_eq!(expected, actual)
}
