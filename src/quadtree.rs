use bevy::prelude::{Component, Vec2};
use noisy_bevy::simplex_noise_2d;

pub(crate) struct QuadTreeBuilder {
    root_size: Vec2,
    root_offset: Vec2,
    unit_size: Vec2,
    noise_scale: f32,
}

impl QuadTreeBuilder {
    pub(crate) fn new(
        root_size: Vec2,
        root_offset: Vec2,
        max_subdivision_count: u32,
        noise_scale: f32,
    ) -> QuadTreeBuilder {
        QuadTreeBuilder {
            root_size,
            root_offset,
            unit_size: root_size / 2_f32.powf(max_subdivision_count as f32),
            noise_scale,
        }
    }

    pub(crate) fn build_root(&self) -> QuadTree {
        self.build(self.root_size, self.root_offset)
    }

    fn build(&self, size: Vec2, offset: Vec2) -> QuadTree {
        if size.length() <= self.unit_size.length() {
            return QuadTree {
                x: offset.x,
                y: offset.y,
                width: size.x,
                height: size.y,
                value: Some(match simplex_noise_2d(self.noise_scale * offset / self.root_size) {
                    value if value > 0. => 1,
                    _ => 0
                }),
                children: vec![],
            };
        }

        let mut children = vec![];
        for x in -1..1 {
            for y in -1..1 {
                let child_size = 0.5 * size;
                let child_offset = offset + 0.5 * child_size + Vec2::new(x as f32, y as f32) * child_size;
                children.push(self.build(child_size, child_offset));
            }
        }

        let first_child = &children[0];
        for child in &children {
            if !first_child.children.is_empty() || !child.children.is_empty() || first_child.value != child.value {
                return QuadTree {
                    x: offset.x,
                    y: offset.y,
                    width: size.x,
                    height: size.y,
                    value: None,
                    children,
                };
            }
        }

        return QuadTree {
            x: offset.x,
            y: offset.y,
            width: size.x,
            height: size.y,
            value: children[0].value,
            children: vec![],
        };
    }
}

#[derive(Component)]
pub(crate) struct QuadTree {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    value: Option<usize>,
    children: Vec<QuadTree>,
}

impl QuadTree {
    pub(crate) fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub(crate) fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    pub(crate) fn value(&self) -> Option<usize> {
        self.value
    }

    pub(crate) fn children(&self) -> &Vec<QuadTree> {
        &self.children
    }
}
