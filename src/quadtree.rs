use bevy::prelude::Vec2;

#[derive(Debug)]
pub struct QuadTree {
    nodes: Vec<QuadTreeNode>,
}

impl QuadTree {
    pub fn new() -> QuadTree {
        QuadTree {
            nodes: vec![
                QuadTreeNode::new(
                    Vec2::new(0.5, 0.5),
                    Vec2::new(1., 1.),
                ),
                QuadTreeNode::new(
                    Vec2::new(0.25, 0.25),
                    Vec2::new(0.5, 0.5),
                ),
                QuadTreeNode::new(
                    Vec2::new(0.75, 0.25),
                    Vec2::new(0.5, 0.5),
                ),
                QuadTreeNode::new(
                    Vec2::new(0.25, 0.75),
                    Vec2::new(0.5, 0.5),
                ),
                QuadTreeNode::new(
                    Vec2::new(0.75, 0.75),
                    Vec2::new(0.5, 0.5),
                ),
            ]
        }
    }

    pub fn leafs(&self) -> impl Iterator<Item=&QuadTreeNode> {
        self.nodes[..1].into_iter()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuadTreeNode {
    center: Vec2,
    size: Vec2,
}

impl QuadTreeNode {
    pub fn new(center: Vec2, size: Vec2) -> QuadTreeNode {
        QuadTreeNode { center, size }
    }
    pub fn center(&self) -> Vec2 { self.center }
    pub fn size(&self) -> Vec2 { self.size }
}