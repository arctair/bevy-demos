use bevy::prelude::Vec2;
use quadtree_demo::quadtree::{QuadTree, QuadTreeNode};

#[test]
fn new_root() {
    let quadtree = QuadTree::new();

    let actual: Vec<QuadTreeNode> = quadtree.leafs().collect();
    let expected = vec![
        QuadTreeNode::new(0b0, 0b0, 0),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn id_root() {
    let id = QuadTreeNode::new(0b0, 0b0, 0);
    assert_eq!(id.center(), Vec2::new(0.5, 0.5));
    assert_eq!(id.size(), Vec2::new(1.0, 1.0));
}

#[test]
fn id_child() {
    let id = QuadTreeNode::new(0b00, 0b00, 1);
    assert_eq!(id.center(), Vec2::new(0.25, 0.25));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));

    let id = QuadTreeNode::new(0b01, 0b00, 1);
    assert_eq!(id.center(), Vec2::new(0.75, 0.25));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));

    let id = QuadTreeNode::new(0b00, 0b01, 1);
    assert_eq!(id.center(), Vec2::new(0.25, 0.75));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));

    let id = QuadTreeNode::new(0b01, 0b01, 1);
    assert_eq!(id.center(), Vec2::new(0.75, 0.75));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));
}

