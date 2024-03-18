use decorum::R32;
use std::collections::HashSet;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut triangle1 = BoundingPolygon::default();
    let mut triangle2 = BoundingPolygon::default();

    let screen_center = vec2(screen_width(), screen_height()) / 2.0;

    let vec_rect1 = vec![vec2(0.0, -30.0), vec2(-30.0, 30.0), vec2(30.0, 30.0)];
    let vec_rect2 = vec![
        screen_center + vec2(0.0, -30.0),
        screen_center + vec2(-30.0, 30.0),
        screen_center + vec2(30.0, 30.0),
    ];

    triangle2.bounding_polygon(&vec_rect2);

    loop {
        clear_background(BLACK);

        let mouse_pos = Vec2::from(mouse_position());

        let v1 = vec_rect1[0] + mouse_pos;
        let v2 = vec_rect1[1] + mouse_pos;
        let v3 = vec_rect1[2] + mouse_pos;
        triangle1.bounding_polygon(&vec![v1, v2, v3]);

        let collides = collides(&triangle1, &triangle2);
        let color = if collides { RED } else { WHITE };

        draw_triangle(v1, v2, v3, color);
        draw_triangle(vec_rect2[0], vec_rect2[1], vec_rect2[2], color);

        next_frame().await
    }
}

#[derive(Default)]
struct BoundingPolygon {
    corners: Vec<Vec2>,
    normals: Vec<Vec2>,
    // center: Vec2,
}

impl BoundingPolygon {
    fn bounding_polygon(&mut self, /* center: Vec2,*/ corners: &Vec<Vec2>) {
        // Store the center and corners
        // self.center = center;
        self.corners = corners.clone();

        // Determine the normal vectors for the sides of the shape
        // let mut normals: HashSet<[i32; 2]> = HashSet::new();
        // let mut normals = BTreeSet::<Vec2>::new();
        let mut normals: HashSet<[R32; 2]> = HashSet::new();

        // Calculate the first edge by subtracting the first from the last corner
        let mut edge = self.corners[self.corners.len() - 1] - self.corners[0];

        // Then determine a perpendicular vector
        let mut perp = vec2(edge.y, -edge.x);

        // Then normalize
        perp = perp.normalize();

        // Add the normal to the list
        normals.insert([perp.x.into(), perp.y.into()]);

        // Repeat for the remaining edges
        for i in 1..self.corners.len() {
            edge = self.corners[i] - self.corners[i - 1];
            perp = vec2(edge.y, -edge.x);
            perp = perp.normalize();
            normals.insert([perp.x.into(), perp.y.into()]);
        }

        // Store the normals
        for r32 in normals.iter() {
            self.normals.push(vec2(r32[0].into(), r32[1].into()));
        }
    }
}

struct MinMax {
    min: f32,
    max: f32,
}

impl MinMax {
    fn new(min: f32, max: f32) -> MinMax {
        MinMax { min, max }
    }
}

fn find_max_min_projection(poly: &BoundingPolygon, axis: Vec2) -> MinMax {
    let mut projection = poly.corners[0].dot(axis);
    let mut max = projection;
    let mut min = projection;
    for i in 1..poly.corners.len() {
        projection = poly.corners[i].dot(axis);
        max = if max > projection { max } else { projection };
        min = if min < projection { min } else { projection };
    }
    MinMax::new(min, max)
}

fn collides(p1: &BoundingPolygon, p2: &BoundingPolygon) -> bool {
    // Check the first polygons's normals
    for normal in &p1.normals {
        // Determine the minimum and maximum projection
        // for both polygons
        let mm1 = find_max_min_projection(p1, *normal);
        let mm2 = find_max_min_projection(p2, *normal);

        // Test for seperation (as soon as we find a seperating axis
        // we know there is no possibility of collision, so we can
        // exit early
        if mm1.max < mm2.min || mm2.max < mm1.min {
            return false;
        }
    }
    // If we reach this point, no seperating axis was found
    // and the two polygons are colliding
    true
}
