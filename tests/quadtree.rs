use bevy::prelude::Vec2;
use quadtree_demo::quadtree::{QuadTree, QuadTreeNode, QuadTreeNodeId};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FillType(u32);

#[test]
fn new_root_no_subdivisions() {
    let quadtree = QuadTree::new(0, |_| FillType(0));

    let actual: Vec<_> = quadtree.nodes().map(|q| *q).collect();
    let expected = vec![
        QuadTreeNode::from((
            QuadTreeNodeId::new(0b0, 0b0, 0),
            FillType(0),
        )),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn one_subdivision_uniform() {
    let quadtree = QuadTree::new(1, |_| FillType(0));

    let actual: Vec<_> = quadtree.nodes().map(|q| *q).collect();
    let expected = vec![
        QuadTreeNode::from((
            QuadTreeNodeId::new(0b0, 0b0, 0),
            FillType(0),
        )),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn one_subdivision_non_uniform() {
    let quadtree = QuadTree::new(1, |position| {
        let left = position.x < 0.5;
        let down = position.y < 0.5;
        if left ^ down {
            FillType(1)
        } else {
            FillType(0)
        }
    });

    let actual: Vec<_> = quadtree.nodes().map(|q| *q).collect();
    let expected = vec![
        QuadTreeNode::from((
            QuadTreeNodeId::new(0b00, 0b00, 1),
            FillType(0),
        )),
        QuadTreeNode::from((
            QuadTreeNodeId::new(0b01, 0b00, 1),
            FillType(1),
        )),
        QuadTreeNode::from((
            QuadTreeNodeId::new(0b01, 0b01, 1),
            FillType(0),
        )),
        QuadTreeNode::from((
            QuadTreeNodeId::new(0b00, 0b01, 1),
            FillType(1),
        )),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn id_root() {
    let id = QuadTreeNodeId::new(0b0, 0b0, 0);
    assert_eq!(id.center(), Vec2::new(0.5, 0.5));
    assert_eq!(id.size(), Vec2::new(1.0, 1.0));
}

#[test]
fn id_child() {
    let id = QuadTreeNodeId::new(0b00, 0b00, 1);
    assert_eq!(id.center(), Vec2::new(0.25, 0.25));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));

    let id = QuadTreeNodeId::new(0b01, 0b00, 1);
    assert_eq!(id.center(), Vec2::new(0.75, 0.25));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));

    let id = QuadTreeNodeId::new(0b00, 0b01, 1);
    assert_eq!(id.center(), Vec2::new(0.25, 0.75));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));

    let id = QuadTreeNodeId::new(0b01, 0b01, 1);
    assert_eq!(id.center(), Vec2::new(0.75, 0.75));
    assert_eq!(id.size(), Vec2::new(0.5, 0.5));
}
