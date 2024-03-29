use slotmap::{new_key_type, SlotMap};

use super::BoundingVolume;

/// Bounding volume hierarchy
///
/// See [crate level documentation](crate)  
pub struct Bvh<D, V: BoundingVolume> {
    arena: SlotMap<NodeIndex, Node<D, V>>,
    root: Option<NodeIndex>,
}

/// Handle of an inserted bounding volume
///
/// It is returned by [`Bvh::insert`], and can be used to remove a bounding volume from the bvh
#[derive(Copy, Clone)]
pub struct VolumeHandle(NodeIndex);

impl<D, V: BoundingVolume> Default for Bvh<D, V> {
    fn default() -> Self {
        Self {
            arena: SlotMap::default(),
            root: None,
        }
    }
}

impl<D, V: BoundingVolume> Bvh<D, V> {
    /// Insert a bounding volume
    ///
    /// It returns an ID that can be u to later remove the volume
    pub fn insert(&mut self, data: D, volume: V) -> VolumeHandle {
        let index = self.arena.insert(Node {
            parent: None,
            volume,
            content: Content::Leaf(data),
        });
        self.root = Some(match self.root {
            Some(root) => insert(&mut self.arena, root, index),
            None => index,
        });
        VolumeHandle(index)
    }

    /// Remove a bounding volume
    ///
    /// It requires the id that was returned by the insert method
    #[allow(clippy::single_match_else, clippy::missing_panics_doc)] // false-positives
    pub fn remove(&mut self, VolumeHandle(node_index): VolumeHandle) {
        match self.arena.remove(node_index).and_then(|n| n.parent) {
            Some(parent_index) => {
                let parent = self
                    .arena
                    .remove(parent_index)
                    .expect("the parent must be in the arena");
                let sibling_index = match parent.content {
                    Content::Tree {
                        left_index,
                        right_index,
                    } if left_index == node_index => right_index,
                    Content::Tree {
                        left_index,
                        right_index,
                    } if right_index == node_index => left_index,
                    _ => {
                        unreachable!("a parent node is always a tree that contains the child node")
                    }
                };

                match parent.parent {
                    // Attach the sibling to the ancestor
                    Some(ancestor) => {
                        self.arena[sibling_index].parent = Some(ancestor);
                        match &mut self.arena[ancestor] {
                            Node {
                                content: Content::Tree { left_index, .. },
                                ..
                            } if *left_index == parent_index => {
                                *left_index = sibling_index;
                            }
                            Node {
                                content: Content::Tree { right_index, .. },
                                ..
                            } if *right_index == parent_index => *right_index = sibling_index,
                            _ => unreachable!(
                                "a parent node is always a tree that contains the child node"
                            ),
                        }
                        recalc_ancestor_volumes(&mut self.arena, ancestor);
                    }
                    // The parent was the root, so the sibling becomes the new root
                    None => {
                        self.arena[sibling_index].parent = None;
                        self.root = Some(sibling_index);
                    }
                }
            }
            // There was no parent, which means it is the root that has been removed
            None => self.root = None,
        }
    }

    /// Clear the hierarchy, leaving it empty
    pub fn clear(&mut self) {
        self.arena.clear();
        self.root = None;
    }

    /// Invokes the given function for each pairs that have overlaping bounding volumes
    pub fn for_each_overlaping_pair(&self, mut on_overlaping_pair: impl FnMut(&D, &D)) {
        if let Some(root) = self.root {
            for_each_overlaping_pair(&self.arena, root, &mut on_overlaping_pair);
        }
    }

    /// Invokes the given function for each volume that overlaps the given `volume`
    pub fn for_each_overlaps(&self, volume: &V, mut on_overlap: impl FnMut(&D)) {
        if let Some(root) = self.root {
            for_each_overlaps(&self.arena, root, volume, &mut on_overlap);
        }
    }
}

new_key_type! {
    struct NodeIndex;
}

struct Node<D, V> {
    parent: Option<NodeIndex>,
    volume: V,
    content: Content<D>,
}

enum Content<D> {
    Leaf(D),
    Tree {
        left_index: NodeIndex,
        right_index: NodeIndex,
    },
}

fn insert<D, V: BoundingVolume>(
    arena: &mut SlotMap<NodeIndex, Node<D, V>>,
    root_index: NodeIndex,
    new_node_index: NodeIndex,
) -> NodeIndex {
    let new_volume = arena[new_node_index].volume;
    match &mut arena[root_index] {
        Node {
            content: Content::Leaf(_),
            volume,
            ..
        } => {
            let new_root_volume = volume.merge(new_volume);
            let new_root_index = arena.insert(Node {
                parent: None,
                volume: new_root_volume,
                content: Content::Tree {
                    left_index: root_index,
                    right_index: new_node_index,
                },
            });
            arena[root_index].parent = Some(new_root_index);
            arena[new_node_index].parent = Some(new_root_index);
            new_root_index
        }
        Node {
            content:
                Content::Tree {
                    left_index,
                    right_index,
                },
            ..
        } => {
            let left_index = *left_index;
            let right_index = *right_index;
            let left_volume = arena[left_index].volume;
            let right_volume = arena[right_index].volume;
            let new_left_volume = left_volume.merge(new_volume);
            let new_right_volume = right_volume.merge(new_volume);
            if new_left_volume.area() - left_volume.area()
                < new_right_volume.area() - right_volume.area()
            {
                let new_left_index = insert(arena, left_index, new_node_index);
                arena[new_left_index].parent = Some(root_index);
                arena[root_index].volume = new_left_volume.merge(right_volume);
                #[allow(clippy::match_wildcard_for_single_variants)]
                match &mut arena[root_index].content {
                    Content::Tree { left_index, .. } => *left_index = new_left_index,
                    _ => unreachable!("we now this is a tree and not a leaf"),
                }
            } else {
                let new_right_index = insert(arena, right_index, new_node_index);
                arena[new_right_index].parent = Some(root_index);
                arena[root_index].volume = new_right_volume.merge(left_volume);
                #[allow(clippy::match_wildcard_for_single_variants)]
                match &mut arena[root_index].content {
                    Content::Tree { right_index, .. } => *right_index = new_right_index,
                    _ => unreachable!("we now this is a tree and not a leaf"),
                }
            }
            root_index
        }
    }
}

fn recalc_ancestor_volumes<D, V: BoundingVolume>(
    arena: &mut SlotMap<NodeIndex, Node<D, V>>,
    node_index: NodeIndex,
) {
    match &arena[node_index] {
        Node {
            parent,
            content:
                Content::Tree {
                    left_index,
                    right_index,
                },
            ..
        } => {
            let parent = *parent;
            arena[node_index].volume = arena[*left_index].volume.merge(arena[*right_index].volume);
            if let Some(p) = parent {
                recalc_ancestor_volumes(arena, p);
            }
        }
        Node {
            content: Content::Leaf(_),
            ..
        } => panic!("a leaf was treated as an ancestor"),
    }
}

fn for_each_overlaping_pair<D, V: BoundingVolume>(
    arena: &SlotMap<NodeIndex, Node<D, V>>,
    node_index: NodeIndex,
    f: &mut impl FnMut(&D, &D),
) {
    if let Content::Tree {
        left_index,
        right_index,
    } = arena[node_index].content
    {
        for_each_overlaping_pair(arena, left_index, f);
        for_each_overlaping_pair(arena, right_index, f);
        for_each_overlaping_pair_between(arena, left_index, right_index, f);
    }
}

fn for_each_overlaping_pair_between<D, V: BoundingVolume>(
    arena: &SlotMap<NodeIndex, Node<D, V>>,
    left_index: NodeIndex,
    right_index: NodeIndex,
    f: &mut impl FnMut(&D, &D),
) {
    let left_node = &arena[left_index];
    let right_node = &arena[right_index];
    if !left_node.volume.overlaps(&right_node.volume) {
        return;
    }
    #[allow(clippy::match_same_arms)]
    match (&left_node.content, &right_node.content) {
        (Content::Leaf(d1), Content::Leaf(d2)) => f(d1, d2),
        (
            Content::Leaf(_),
            Content::Tree {
                left_index: right_left,
                right_index: right_right,
            },
        ) => {
            for_each_overlaping_pair_between(arena, left_index, *right_left, f);
            for_each_overlaping_pair_between(arena, left_index, *right_right, f);
        }
        (
            Content::Tree {
                left_index: left_left,
                right_index: left_right,
            },
            Content::Leaf(_),
        ) => {
            for_each_overlaping_pair_between(arena, right_index, *left_left, f);
            for_each_overlaping_pair_between(arena, right_index, *left_right, f);
        }
        (
            Content::Tree { .. },
            Content::Tree {
                left_index: right_left,
                right_index: right_right,
            },
        ) if left_node.volume.area() < right_node.volume.area() => {
            for_each_overlaping_pair_between(arena, left_index, *right_left, f);
            for_each_overlaping_pair_between(arena, left_index, *right_right, f);
        }
        (
            Content::Tree {
                left_index: left_left,
                right_index: left_right,
            },
            Content::Tree { .. },
        ) => {
            for_each_overlaping_pair_between(arena, right_index, *left_left, f);
            for_each_overlaping_pair_between(arena, right_index, *left_right, f);
        }
    }
}

fn for_each_overlaps<D, V: BoundingVolume>(
    arena: &SlotMap<NodeIndex, Node<D, V>>,
    node_index: NodeIndex,
    volume: &V,
    f: &mut impl FnMut(&D),
) {
    let node = &arena[node_index];
    if !node.volume.overlaps(volume) {
        return;
    }
    match &node.content {
        Content::Leaf(d) => f(d),
        Content::Tree {
            left_index,
            right_index,
        } => {
            for_each_overlaps(arena, *left_index, volume, f);
            for_each_overlaps(arena, *right_index, volume, f);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::volumes::Aabb;

    use super::*;

    #[test]
    fn update_parent_volume_on_insertion() {
        let mut bvh: Bvh<(), Aabb<1>> = Bvh::default();
        bvh.insert((), Aabb::from_min_max([0.0], [1.0]));
        bvh.insert((), Aabb::from_min_max([2.0], [3.0]));
        bvh.insert((), Aabb::from_min_max([3.0], [4.0]));
        bvh.insert((), Aabb::from_min_max([2.0], [5.0]));
        assert_eq!(
            bvh.arena[bvh.root.expect("must have a root")].volume,
            Aabb::from_min_max([0.0], [5.0])
        );
    }

    #[test]
    fn update_parent_volume_on_removal() {
        let mut bvh: Bvh<(), Aabb<1>> = Bvh::default();
        bvh.insert((), Aabb::from_min_max([0.0], [1.0]));
        bvh.insert((), Aabb::from_min_max([2.0], [3.0]));
        bvh.insert((), Aabb::from_min_max([3.0], [4.0]));
        let id = bvh.insert((), Aabb::from_min_max([2.0], [5.0]));
        bvh.remove(id);
        assert_eq!(
            bvh.arena[bvh.root.expect("must have a root")].volume,
            Aabb::from_min_max([0.0], [4.0])
        );
    }
}
