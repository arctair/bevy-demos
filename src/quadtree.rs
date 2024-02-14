use std::slice::Iter;
use bevy::prelude::Vec2;

#[derive(Debug)]
pub struct QuadTree<T> {
    nodes: Vec<QuadTreeNode<T>>,
}

impl<T> QuadTree<T> {
    pub fn new(default_value: T) -> QuadTree<T> {
        let root = QuadTreeNode::new(0b0, 0b0, 0, default_value);
        QuadTree { nodes: vec![root] }
    }

    pub fn sample(&mut self, from: fn() -> T) {
        let root = &mut self.nodes[0];
        root.value = from();
    }

    pub fn leafs(&self) -> Iter<'_, QuadTreeNode<T>> {
        self.nodes.iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadTreeNode<T> {
    x: i32,
    y: i32,
    depth: i32,
    value: T,
}

impl<T> QuadTreeNode<T> {
    pub fn new(x: i32, y: i32, depth: i32, value: T) -> QuadTreeNode<T> {
        QuadTreeNode { x, y, depth, value }
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