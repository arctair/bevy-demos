use bevy::prelude::Vec2;
use quadtree_demo::quadtree::{QuadTree, QuadTreeNode};

#[derive(Copy, Clone, Debug, PartialEq)]
struct FillType(u32);

#[test]
fn new_root_no_subdivisions_default_value_then_sample_value() {
    let mut quadtree = QuadTree::new(FillType(0));

    let actual: Vec<_> = quadtree.leafs().map(|q| *q).collect();
    let expected = vec![
        QuadTreeNode::new(0b0, 0b0, 0, FillType(0)),
    ];

    assert_eq!(actual, expected);

    quadtree.sample(|| FillType(1));

    let actual: Vec<_> = quadtree.leafs().map(|q| *q).collect();
    let expected = vec![
        QuadTreeNode::new(0b00, 0b00, 0, FillType(1)),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn id_root() {
    let id = QuadTreeNode::new(0b0, 0b0, 0, ());
    assert_eq!(id.center(), Vec2::new(0.5, 0.5));
    assert_eq!(id.size(), Vec2::new(1.0, 1.0));
}

#[test]
fn id_child() {
    let id = QuadTreeNode::new(0b00, 0b00, 1, ());
    assert_eq!(id.center(), Vec2::new(0.25, 0.25));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));

    let id = QuadTreeNode::new(0b01, 0b00, 1, ());
    assert_eq!(id.center(), Vec2::new(0.75, 0.25));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));

    let id = QuadTreeNode::new(0b00, 0b01, 1, ());
    assert_eq!(id.center(), Vec2::new(0.25, 0.75));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));

    let id = QuadTreeNode::new(0b01, 0b01, 1, ());
    assert_eq!(id.center(), Vec2::new(0.75, 0.75));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));
}
