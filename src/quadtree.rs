use bevy::prelude::Vec2;

#[derive(Debug)]
pub struct QuadTree {
    nodes: Vec<QuadTreeNode>,
}

impl QuadTree {
    pub fn new() -> QuadTree {
        QuadTree {
            nodes: vec![
                QuadTreeNode::new(0b0, 0b0, 0),
            ]
        }
    }

    pub fn leafs(&self) -> impl Iterator<Item=QuadTreeNode> + '_ {
        self.nodes.iter().map(|q| q.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuadTreeNode {
    x: i32,
    y: i32,
    depth: i32,
}

impl QuadTreeNode {
    pub fn new(x: i32, y: i32, depth: i32) -> QuadTreeNode {
        QuadTreeNode { x, y, depth }
    }

    pub fn center(&self) -> Vec2 {
        self.size() * Vec2::new(
            0.5 + self.x as f32,
            0.5 + self.y as f32,
        )
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(1.0, 1.0) / 2_f32.powf(self.depth as f32)
    }
}