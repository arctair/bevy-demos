use bevy::prelude::Vec2;

pub struct QuadTree {
    nodes: Vec<QuadTreeNode>,
}

impl QuadTree {
    pub fn new() -> QuadTree {
        QuadTree {
            nodes: vec![
                QuadTreeNode {
                    center: Vec2::new(0.5, 0.5),
                    size: Vec2::new(1., 1.),
                },
                QuadTreeNode {
                    center: Vec2::new(0.25, 0.25),
                    size: Vec2::new(0.5, 0.5),
                },
                QuadTreeNode {
                    center: Vec2::new(0.75, 0.25),
                    size: Vec2::new(0.5, 0.5),
                },
                QuadTreeNode {
                    center: Vec2::new(0.25, 0.75),
                    size: Vec2::new(0.5, 0.5),
                },
                QuadTreeNode {
                    center: Vec2::new(0.75, 0.75),
                    size: Vec2::new(0.5, 0.5),
                },
            ]
        }
    }

    pub fn leafs(&self) -> impl Iterator<Item=&QuadTreeNode> {
        self.nodes[1..].into_iter()
    }
}

pub struct QuadTreeNode {
    center: Vec2,
    size: Vec2,
}

impl QuadTreeNode {
    pub fn center(&self) -> Vec2 { self.center }
    pub fn size(&self) -> Vec2 { self.size }
}