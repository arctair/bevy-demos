use bevy::prelude::{Component, Vec2};
use bevy_rapier2d::prelude::{Collider, Rot, Vect};
use noisy_bevy::simplex_noise_2d;

pub(crate) struct SampleIntoQuadTree {
    noise_scale: f32,
    quadtree: QuadTree,
}

impl SampleIntoQuadTree {
    pub(crate) fn new(
        size: Vec2,
        position: Vec2,
        max_subdivision_count: u32,
        noise_scale: f32,
    ) -> SampleIntoQuadTree {
        SampleIntoQuadTree {
            noise_scale,
            quadtree: QuadTree {
                unit_degree: max_subdivision_count,
                x: position.x,
                y: position.y,
                width: size.x,
                height: size.y,
                value: None,
                children: vec![],
            },
        }
    }

    pub(crate) fn build(mut self) -> QuadTree {
        let top_left = self.quadtree.position() - 0.5 * self.quadtree.size();
        let row_count = 2_u32.pow(self.quadtree.unit_degree);
        let unit_length = self.quadtree.width / row_count as f32;
        for x in 0..row_count {
            for y in 0..row_count {
                let position = top_left + unit_length * Vec2::new(0.5 + x as f32, 0.5 + y as f32);
                let value = match simplex_noise_2d(self.noise_scale * position / self.quadtree.size()) {
                    value if value > 0. => 1,
                    _ => 0
                };
                self.quadtree.set_value(position, 0.5 * unit_length, value)
            }
        }
        self.quadtree
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
        if self.intersects_circle(position, radius) {
            if self.unit_degree <= 0 {
                self.value = Some(value);
            } else if self.value != Some(value) && self.children.is_empty() {
                self.subdivide();
                self.value = None;
            }

            for child in &mut self.children {
                child.set_value(position, radius, value)
            }

            self.consolidate()
        }
    }

    fn intersects_circle(&self, position: Vec2, radius: f32) -> bool {
        let delta = position - self.position();
        let half_width = self.width / 2.;
        let half_height = self.height / 2.;
        let closest = self.position() + Vec2::new(
            half_width.min((-half_width).max(delta.x)),
            half_height.min((-half_height).max(delta.y)),
        );
        let distance = closest.distance(position);
        distance < radius
    }

    fn subdivide(&mut self) {
        for x in -1..1 {
            for y in -1..1 {
                let child_size = 0.5 * self.size();
                let child_offset = self.position() + 0.5 * child_size + Vec2::new(x as f32, y as f32) * child_size;
                self.children.push(QuadTree {
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
    }

    fn consolidate(&mut self) {
        if let Some(first_child) = self.children.first() {
            if first_child.children.is_empty() && self.children.iter().all(|child| child.children.is_empty() && first_child.value == child.value) {
                self.value = first_child.value;
                self.children = vec![];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;
    use crate::quadtree::SampleIntoQuadTree;

    #[test]
    fn test_set_value_0_subdivisions_hit() {
        for (target, radius) in vec![
            (Vec2::ZERO, 0.1),
            (Vec2::new(1.9, 0.), 1.0),
            (Vec2::splat(1.0 + 1.0 / 2.0f32.sqrt() - 0.1), 1.0),
        ] {
            let mut root = SampleIntoQuadTree::new(
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
            let mut root = SampleIntoQuadTree::new(
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

        let mut root = SampleIntoQuadTree::new(
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

        let mut root = SampleIntoQuadTree::new(
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

        let mut root = SampleIntoQuadTree::new(
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
        let mut root = SampleIntoQuadTree::new(
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