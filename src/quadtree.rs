use std::slice::Iter;
use bevy::prelude::Vec2;

#[derive(Debug)]
pub struct QuadTree<T> {
    nodes: Vec<QuadTreeNode<T>>,
}

impl<T: Eq> QuadTree<T> {
    pub fn new(subdivisions: usize, from: fn(Vec2) -> T) -> QuadTree<T> {
        let root_id = QuadTreeNodeId::new(0, 0, 0);
        let root = QuadTreeNode::from((root_id, from));
        let mut nodes = vec![root];
        let mut index = 0;
        loop {
            if index > 3 && {
                let mut uniform = true;

                let first = &&nodes[index - 4];
                for offset in 0..3 {
                    let node = &&nodes[index + offset - 3];
                    if (node.id.x >> 1) != (first.id.x >> 1)
                        || (node.id.y >> 1) != (first.id.y >> 1)
                        || node.value != first.value { uniform = false; }
                }

                uniform
            } {
                // COMPACT : consolidate uniform regions that formed just before index
                let first = nodes.remove(index - 4);
                nodes.remove(index - 4);
                nodes.remove(index - 4);
                nodes.remove(index - 4);
                nodes.insert(index - 4, QuadTreeNode::from((QuadTreeNodeId::new(
                    first.id.x >> 1,
                    first.id.y >> 1,
                    first.id.depth - 1,
                ), first.value)));
                index = index - 3;
            } else if let Some(node) = nodes.get(index) {
                // SUBDIVIDE : replace this region at this index with quarter regions and resample
                if node.id.depth >= subdivisions {
                    index += 1;
                } else {
                    let left = (node.id.x << 1) + 0;
                    let right = (node.id.x << 1) + 1;
                    let bottom = (node.id.y << 1) + 0;
                    let top = (node.id.y << 1) + 1;
                    let depth = node.id.depth + 1;

                    nodes.remove(index);
                    nodes.append(&mut vec![
                        QuadTreeNode::from((QuadTreeNodeId::new(left, bottom, depth), from)),
                        QuadTreeNode::from((QuadTreeNodeId::new(right, bottom, depth), from)),
                        QuadTreeNode::from((QuadTreeNodeId::new(right, top, depth), from)),
                        QuadTreeNode::from((QuadTreeNodeId::new(left, top, depth), from)),
                    ]);
                }
            } else {
                break;
            }
        }
        QuadTree { nodes }
    }

    pub fn nodes(&self) -> Iter<'_, QuadTreeNode<T>> {
        self.nodes.iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadTreeNode<T> {
    pub id: QuadTreeNodeId,
    value: T,
}

impl<T> From<(QuadTreeNodeId, T)> for QuadTreeNode<T> {
    fn from((id, value): (QuadTreeNodeId, T)) -> Self {
        Self { id, value }
    }
}

impl<T> From<(QuadTreeNodeId, fn(Vec2) -> T)> for QuadTreeNode<T> {
    fn from((id, get_value): (QuadTreeNodeId, fn(Vec2) -> T)) -> Self {
        Self { id, value: get_value(id.center()) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadTreeNodeId {
    x: i32,
    y: i32,
    depth: usize,
}

impl QuadTreeNodeId {
    pub fn new(x: i32, y: i32, depth: usize) -> QuadTreeNodeId {
        QuadTreeNodeId { x, y, depth }
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