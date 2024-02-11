use bevy::prelude::{Component, Vec2};
use bevy_rapier2d::prelude::{Collider, Rot, Vect};
use noisy_bevy::simplex_noise_2d;

pub(crate) struct QuadTreeBuilder {
    root_size: Vec2,
    root_offset: Vec2,
    max_subdivision_count: u32,
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
            max_subdivision_count,
            noise_scale,
        }
    }

    pub(crate) fn build(&self) -> QuadTree {
        let mut root = QuadTree {
            unit_degree: self.max_subdivision_count,
            x: self.root_offset.x,
            y: self.root_offset.y,
            width: self.root_size.x,
            height: self.root_size.y,
            value: None,
            children: vec![],
        };
        self.sample_into(&mut root);
        root
    }

    fn sample_into(&self, quadtree: &mut QuadTree) {
        if quadtree.unit_degree == 0 {
            quadtree.value = Some(match simplex_noise_2d(self.noise_scale * quadtree.position() / self.root_size) {
                value if value > 0. => 1,
                _ => 0
            })
        } else {
            quadtree.subdivide();
            for child in &mut quadtree.children {
                self.sample_into(child);
            }
            quadtree.consolidate();
        }
    }
}

#[derive(Component)]
pub(crate) struct QuadTree {
    unit_degree: u32,
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

    pub(crate) fn children(&self) -> &Vec<QuadTree> {
        &self.children
    }

    pub(crate) fn value(&self) -> Option<usize> {
        self.value
    }

    pub(crate) fn collider(&self) -> Collider {
        Collider::compound(self.colliders())
    }

    fn colliders(&self) -> Vec<(Vect, Rot, Collider)> {
        if self.children.is_empty() {
            match self.value {
                Some(1) => vec![(Vect::new(self.x, self.y), 0., Collider::cuboid(self.width / 2., self.height / 2.))],
                _ => vec![]
            }
        } else {
            self.children.iter().flat_map(QuadTree::colliders).collect()
        }
    }

    pub(crate) fn set_value(&mut self, position: Vec2, radius: f32, value: usize) {
        let delta = position - self.position();
        let half_width = self.width / 2.;
        let half_height = self.height / 2.;
        let closest = self.position() + Vec2::new(
            half_width.min((-half_width).max(delta.x)),
            half_height.min((-half_height).max(delta.y)),
        );
        let distance = closest.distance(position);
        if distance < radius {
            if self.unit_degree <= 0 {
                self.value = Some(value);
            } else if self.value != Some(value) && self.children.is_empty() {
                self.subdivide()
            }

            for child in &mut self.children {
                child.set_value(position, radius, value)
            }

            self.consolidate()
        }
    }

    fn subdivide(&mut self) {
        let mut children = vec![];
        for x in -1..1 {
            for y in -1..1 {
                let child_size = 0.5 * self.size();
                let child_offset = self.position() + 0.5 * child_size + Vec2::new(x as f32, y as f32) * child_size;
                children.push(QuadTree {
                    x: child_offset.x,
                    y: child_offset.y,
                    width: child_size.x,
                    height: child_size.y,
                    unit_degree: self.unit_degree - 1,
                    value: self.value,
                    children: vec![],
                });
            }
        }
        self.children = children;
        self.value = None;
    }

    fn consolidate(&mut self) {
        if let Some(first_child) = self.children.first() {
            for child in &self.children {
                if !first_child.children.is_empty() || !child.children.is_empty() || first_child.value != child.value {
                    return;
                }
            }

            self.value = first_child.value;
            self.children = vec![];
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;
    use crate::quadtree::QuadTreeBuilder;

    #[test]
    fn test_set_value_0_subdivisions_hit() {
        for (target, radius) in vec![
            (Vec2::ZERO, 0.1),
            (Vec2::new(1.9, 0.), 1.0),
            (Vec2::splat(1.0 + 1.0 / 2.0f32.sqrt() - 0.1), 1.0),
        ] {
            let mut root = QuadTreeBuilder::new(
                Vec2::new(2., 2.),
                Vec2::ZERO,
                0,
                1.,
            ).build();

            assert_eq!(root.value, Some(0));

            root.set_value(target, radius, 1);

            assert_eq!(root.value, Some(1), "failed to set value for target {target} radius {radius}");
        }
    }

    #[test]
    fn test_set_value_0_subdivisions_miss() {
        for (target, radius) in vec![
            (Vec2::new(2., 0.), 1.0),
            (Vec2::splat(1.0 + 1.0 / 2.0_f32.sqrt()), 1.0),
        ] {
            let mut root = QuadTreeBuilder::new(
                Vec2::new(2., 2.),
                Vec2::ZERO,
                0,
                1.,
            ).build();

            assert_eq!(root.value, Some(0));

            root.set_value(target, radius, 1);

            assert_eq!(root.value, Some(0), "mistakenly set value for target {target} radius {radius}");
        }
    }

    #[test]
    fn test_set_value_must_subdivide() {
        let (target, radius) = (Vec2::new(-0.5, -0.5), 0.1);

        let mut root = QuadTreeBuilder::new(
            Vec2::new(2., 2.),
            Vec2::ZERO,
            1,
            0.,
        ).build();

        assert_eq!(root.value, Some(0));

        root.set_value(target, radius, 1);

        assert_eq!(root.children.len(), 4, "failed to subdivide root");
        assert_eq!(root.value, None, "failed to unset root value after subdivision ");
        assert_eq!(root.children[0].value, Some(1), "failed to set SW child value for target {target} radius {radius}");
        assert_eq!(root.children[1].value, Some(0), "failed to set NW child value to old root value");
        assert_eq!(root.children[2].value, Some(0), "failed to set SE child value to old root value");
        assert_eq!(root.children[3].value, Some(0), "failed to set NE child value to old root value");
    }

    #[test]
    fn test_set_value_superdivision_already_set() {
        let (target, radius) = (Vec2::new(-0.5, -0.5), 0.1);

        let mut root = QuadTreeBuilder::new(
            Vec2::new(2., 2.),
            Vec2::ZERO,
            1,
            0.,
        ).build();

        assert_eq!(root.value, Some(0));

        root.set_value(target, radius, 0);

        assert_eq!(root.value, Some(0));
    }

    #[test]
    fn test_set_value_already_subdivided() {
        let (target, radius) = (Vec2::new(-0.5, -0.5), 0.1);

        let mut root = QuadTreeBuilder::new(
            Vec2::new(2., 2.),
            Vec2::ZERO,
            1,
            1.,
        ).build();

        assert_eq!(root.children[0].value, Some(0));
        assert_eq!(root.children[1].value, Some(1));
        assert_eq!(root.children[2].value, Some(0));
        assert_eq!(root.children[3].value, Some(1));

        root.set_value(target, radius, 1);

        assert_eq!(root.children[0].value, Some(1), "failed to set SW child value for target {target} radius {radius}");
        assert_eq!(root.children[1].value, Some(1), "mistakenly set NW child value");
        assert_eq!(root.children[2].value, Some(0), "mistakenly set SE child value");
        assert_eq!(root.children[3].value, Some(1), "mistakenly set NE child value");
    }

    #[test]
    fn test_set_value_and_consolidate() {
        let mut root = QuadTreeBuilder::new(
            Vec2::new(2., 2.),
            Vec2::ZERO,
            1,
            1.,
        ).build();

        assert_eq!(root.children[0].value, Some(0));
        assert_eq!(root.children[1].value, Some(1));
        assert_eq!(root.children[2].value, Some(0));
        assert_eq!(root.children[3].value, Some(1));

        root.set_value(Vec2::new(-0.5, -0.5), 0.1, 1);
        root.set_value(Vec2::new(0.5, -0.5), 0.1, 1);

        assert_eq!(root.children.is_empty(), true);
        assert_eq!(root.value, Some(1));
    }
}