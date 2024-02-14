use bevy::prelude::Vec2;
use quadtree_demo::quadtree::{QuadTree, QuadTreeNode};

#[test]
fn new_root() {
    let quadtree = QuadTree::new();

    let actual: Vec<QuadTreeNode> = quadtree.leafs().map(QuadTreeNode::clone).collect();
    let expected = vec![
        QuadTreeNode::new(
            Vec2::new(0.5, 0.5),
            Vec2::new(1.0, 1.0),
        ),
    ];

    assert_eq!(actual, expected);
}