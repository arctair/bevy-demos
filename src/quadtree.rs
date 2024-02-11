use bevy::prelude::{Component, Vec2};
use bevy::utils::Uuid;

#[derive(Component)]
pub(crate) struct QuadTree {
    id: QuadTreeId,
    pub(crate) unit_degree: u32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    value: Option<usize>,
    children: Vec<QuadTree>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct QuadTreeEvent {
    pub(crate) quadtree_id: QuadTreeId,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) value: Option<usize>,
}

#[derive(Clone, Component, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct QuadTreeId(Uuid);

impl QuadTree {
    pub(crate) fn new(
        size: Vec2,
        position: Vec2,
        max_subdivision_count: u32,
    ) -> Self {
        QuadTree {
            id: QuadTreeId(Uuid::new_v4()),
            unit_degree: max_subdivision_count,
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
            value: None,
            children: vec![],
        }
    }

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

    pub(crate) fn set_value(&mut self, position: Vec2, radius: f32, value: usize) -> Vec<QuadTreeEvent> {
        let mut events = vec![];
        if self.intersects_circle(position, radius) {
            if self.value != Some(value) {
                if self.unit_degree <= 0 {
                    self.value = Some(value);
                    events.push(QuadTreeEvent {
                        quadtree_id: self.id,
                        x: self.x,
                        y: self.y,
                        width: self.width,
                        height: self.height,
                        value: Some(value),
                    })
                } else if self.children.is_empty() {
                    events.append(&mut self.subdivide());

                    self.value = None;
                    events.push(QuadTreeEvent {
                        quadtree_id: self.id,
                        x: self.x,
                        y: self.y,
                        width: self.width,
                        height: self.height,
                        value: None,
                    });
                }
            }

            for child in &mut self.children {
                events.append(&mut child.set_value(position, radius, value));
            }

            events.append(&mut self.consolidate());
        }
        events
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

    fn subdivide(&mut self) -> Vec<QuadTreeEvent> {
        let mut events = vec![];
        for x in -1..1 {
            for y in -1..1 {
                let child_size = 0.5 * self.size();
                let child_offset = self.position() + 0.5 * child_size + Vec2::new(x as f32, y as f32) * child_size;
                let id = QuadTreeId(Uuid::new_v4());
                self.children.push(QuadTree {
                    id,
                    x: child_offset.x,
                    y: child_offset.y,
                    width: child_size.x,
                    height: child_size.y,
                    unit_degree: self.unit_degree - 1,
                    value: self.value,
                    children: vec![],
                });
                events.push(QuadTreeEvent {
                    quadtree_id: id,
                    x: child_offset.x,
                    y: child_offset.y,
                    width: child_size.x,
                    height: child_size.y,
                    value: self.value,
                });
            }
        }
        events
    }

    fn consolidate(&mut self) -> Vec<QuadTreeEvent> {
        let mut events = vec![];
        if let Some(first_child) = self.children.first() {
            if first_child.children.is_empty() && self.children.iter().all(|child| child.children.is_empty() && first_child.value == child.value) {
                self.value = first_child.value;
                events.push(QuadTreeEvent {
                    quadtree_id: self.id,
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: self.height,
                    value: self.value,
                });

                for child in &self.children {
                    events.push(QuadTreeEvent {
                        quadtree_id: child.id,
                        x: child.x,
                        y: child.y,
                        width: child.width,
                        height: child.height,
                        value: None,
                    });
                }
                self.children = vec![];
            }
        }
        events
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;
    use crate::quadtree::{QuadTree, QuadTreeEvent, QuadTreeId};

    #[test]
    fn test_set_value_0_subdivisions_hit() {
        for (target, radius) in vec![
            (Vec2::ZERO, 0.1),
            (Vec2::new(1.9, 0.), 1.0),
            (Vec2::splat(1.0 + 1.0 / 2.0f32.sqrt() - 0.1), 1.0),
        ] {
            let mut root = QuadTree::new(
                Vec2::new(2., 2.),
                Vec2::ZERO,
                0,
            );

            root.set_value(Vec2::ZERO, 1., 0);

            assert_eq!(root.value, Some(0));

            let actual_events = root.set_value(target, radius, 1);
            let expected_events = vec![
                QuadTreeEvent {
                    quadtree_id: root.id,
                    x: root.x,
                    y: root.y,
                    width: root.width,
                    height: root.height,
                    value: Some(1),
                }
            ];
            assert_eq!(actual_events, expected_events);

            assert_eq!(root.value, Some(1), "failed to set value for target {target} radius {radius}");
        }
    }

    #[test]
    fn test_set_value_0_subdivisions_hit_value_match() {
        let mut root = QuadTree::new(
            Vec2::new(2., 2.),
            Vec2::ZERO,
            0,
        );

        root.set_value(Vec2::ZERO, 1., 0);

        assert_eq!(root.value, Some(0));

        let actual_events = root.set_value(Vec2::ZERO, 0.1, 0);
        let expected_events = vec![];
        assert_eq!(actual_events, expected_events);

        assert_eq!(root.value, Some(0));
    }

    #[test]
    fn test_set_value_0_subdivisions_miss() {
        for (target, radius) in vec![
            (Vec2::new(2., 0.), 1.0),
            (Vec2::splat(1.0 + 1.0 / 2.0_f32.sqrt()), 1.0),
        ] {
            let mut root = QuadTree::new(
                Vec2::new(2., 2.),
                Vec2::ZERO,
                0,
            );

            root.set_value(Vec2::ZERO, 1., 0);

            assert_eq!(root.value, Some(0));

            let actual_events = root.set_value(target, radius, 1);
            let expected_events = vec![];
            assert_eq!(actual_events, expected_events);

            assert_eq!(root.value, Some(0), "mistakenly set value for target {target} radius {radius}");
        }
    }

    #[test]
    fn test_set_value_must_subdivide() {
        let mut root = QuadTree::new(
            Vec2::new(2., 2.),
            Vec2::ZERO,
            1,
        );

        root.set_value(Vec2::ZERO, 1., 0);

        assert_eq!(root.value, Some(0));

        let actual_events = root.set_value(Vec2::new(-0.5, -0.5), 0.1, 1);
        let expected_events = vec![
            QuadTreeEvent {
                quadtree_id: root.children[0].id,
                x: root.children[0].x,
                y: root.children[0].y,
                width: root.children[0].width,
                height: root.children[0].height,
                value: Some(0),
            },
            QuadTreeEvent {
                quadtree_id: root.children[1].id,
                x: root.children[1].x,
                y: root.children[1].y,
                width: root.children[1].width,
                height: root.children[1].height,
                value: Some(0),
            },
            QuadTreeEvent {
                quadtree_id: root.children[2].id,
                x: root.children[2].x,
                y: root.children[2].y,
                width: root.children[2].width,
                height: root.children[2].height,
                value: Some(0),
            },
            QuadTreeEvent {
                quadtree_id: root.children[3].id,
                x: root.children[3].x,
                y: root.children[3].y,
                width: root.children[3].width,
                height: root.children[3].height,
                value: Some(0),
            },
            QuadTreeEvent {
                quadtree_id: root.id,
                x: root.x,
                y: root.y,
                width: root.width,
                height: root.height,
                value: None,
            },
            QuadTreeEvent {
                quadtree_id: root.children[0].id,
                x: root.children[0].x,
                y: root.children[0].y,
                width: root.children[0].width,
                height: root.children[0].height,
                value: Some(1),
            },
        ];
        assert_eq!(actual_events, expected_events);

        assert_eq!(root.children.is_empty(), false, "failed to subdivide root");
        assert_eq!(root.value, None, "failed to unset root value after subdivision ");
        assert_eq!(root.children[0].value, Some(1));
        assert_eq!(root.children[1].value, Some(0));
        assert_eq!(root.children[2].value, Some(0));
        assert_eq!(root.children[3].value, Some(0));
    }

    #[test]
    fn test_set_value_superdivision_already_set() {
        let (target, radius) = (Vec2::new(-0.5, -0.5), 0.1);

        let mut root = QuadTree::new(
            Vec2::new(2., 2.),
            Vec2::ZERO,
            1,
        );

        root.set_value(Vec2::ZERO, 1., 0);

        assert_eq!(root.value, Some(0));

        let actual_events = root.set_value(target, radius, 0);
        let expected_events = vec![];
        assert_eq!(actual_events, expected_events);

        assert_eq!(root.value, Some(0));
    }

    #[test]
    fn test_set_value_already_subdivided() {
        let (target, radius) = (Vec2::new(-0.5, -0.5), 0.1);

        let mut root = QuadTree::new(
            Vec2::new(2., 2.),
            Vec2::ZERO,
            1,
        );

        root.set_value(Vec2::new(0., -0.5), 0.5, 0);
        root.set_value(Vec2::new(0., 0.5), 0.5, 1);

        assert_eq!(root.children[0].value, Some(0));
        assert_eq!(root.children[1].value, Some(1));
        assert_eq!(root.children[2].value, Some(0));
        assert_eq!(root.children[3].value, Some(1));

        let actual_events = root.set_value(target, radius, 1);
        let expected_events = vec![
            QuadTreeEvent {
                quadtree_id: root.children[0].id,
                x: root.children[0].x,
                y: root.children[0].y,
                width: root.children[0].width,
                height: root.children[0].height,
                value: Some(1),
            },
        ];
        assert_eq!(actual_events, expected_events);

        assert_eq!(root.children[0].value, Some(1), "failed to set SW child value for target {target} radius {radius}");
        assert_eq!(root.children[1].value, Some(1), "mistakenly set NW child value");
        assert_eq!(root.children[2].value, Some(0), "mistakenly set SE child value");
        assert_eq!(root.children[3].value, Some(1), "mistakenly set NE child value");
    }

    #[test]
    fn test_set_value_and_consolidate() {
        let mut root = QuadTree::new(
            Vec2::new(2., 2.),
            Vec2::ZERO,
            1,
        );

        root.set_value(Vec2::new(0., -0.5), 0.5, 0);
        root.set_value(Vec2::new(0., 0.5), 0.5, 1);

        assert_eq!(root.children[0].value, Some(0));
        assert_eq!(root.children[1].value, Some(1));
        assert_eq!(root.children[2].value, Some(0));
        assert_eq!(root.children[3].value, Some(1));

        let ids: Vec<QuadTreeId> = root.children.iter().map(|q| q.id).collect();

        let actual_events = root.set_value(Vec2::new(-0.5, -0.5), 0.1, 1);
        let expected_events = vec![
            QuadTreeEvent {
                quadtree_id: root.children[0].id,
                x: root.children[0].x,
                y: root.children[0].y,
                width: root.children[0].width,
                height: root.children[0].height,
                value: Some(1),
            },
        ];
        assert_eq!(actual_events, expected_events);

        let actual_events = root.set_value(Vec2::new(0.5, -0.5), 0.1, 1);
        let expected_events = vec![
            QuadTreeEvent {
                quadtree_id: ids[2],
                x: 0.5,
                y: -0.5,
                width: 1.0,
                height: 1.0,
                value: Some(1),
            },
            QuadTreeEvent {
                quadtree_id: root.id,
                x: root.x,
                y: root.y,
                width: root.width,
                height: root.height,
                value: Some(1),
            },
            QuadTreeEvent {
                quadtree_id: ids[0],
                x: -0.5,
                y: -0.5,
                width: 1.0,
                height: 1.0,
                value: None,
            },
            QuadTreeEvent {
                quadtree_id: ids[1],
                x: -0.5,
                y: 0.5,
                width: 1.0,
                height: 1.0,
                value: None,
            },
            QuadTreeEvent {
                quadtree_id: ids[2],
                x: 0.5,
                y: -0.5,
                width: 1.0,
                height: 1.0,
                value: None,
            },
            QuadTreeEvent {
                quadtree_id: ids[3],
                x: 0.5,
                y: 0.5,
                width: 1.0,
                height: 1.0,
                value: None,
            },
        ];
        assert_eq!(actual_events, expected_events);

        assert_eq!(root.children.is_empty(), true);
        assert_eq!(root.value, Some(1));
    }
}