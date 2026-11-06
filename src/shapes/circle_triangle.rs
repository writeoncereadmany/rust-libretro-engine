use super::projection::{collision_on_axis, intersects_on_axis, intersects_on_axis_moving, Projects};
use crate::shapes::circle;
use crate::shapes::circle::Circle;
use crate::shapes::collision::Collision;
use crate::shapes::triangle::{translate, Triangle};
use crate::shapes::vec2d::{nearest, Vec2d, UNIT_X, UNIT_Y};
use itertools::Itertools;
use std::collections::VecDeque;

pub fn intersects(triangle: &Triangle, circle: &Circle) -> bool {
    triangle.normals.iter().all(|axis| intersects_on_axis(triangle, circle, axis)) && {
        let closest_corner = nearest_corner(&circle.center, triangle);
        let separation = circle.center.sub(&closest_corner);
        if separation.sq_len() == 0.0 {
            true
        } else {
            let closest_corner_axis = separation.unit();
            intersects_on_axis(triangle, circle, &closest_corner_axis)
        }
    }
}

pub fn intersects_moving(triangle: &Triangle, circle: &Circle, dv: &(f64, f64)) -> bool {
    if dv.sq_len() == 0.0 {
        return intersects(triangle, circle);
    }

    triangle.normals.iter().all(|axis| intersects_on_axis_moving(triangle, circle, dv, axis)) && {
        let normal_to_travel = dv.unit().perpendicular();
        // no movement normal to the direction of movement, so we can just use intersects,
        // instead of intersects moving.
        intersects_on_axis(triangle, circle, &normal_to_travel)
    } && {
        let nearest_corner = nearest(&circle.center, &[
            nearest_corner(&circle.center, triangle),
            nearest_corner(&circle.center, &translate(triangle, dv))
        ]);
        let separation = circle.center.sub(&nearest_corner);
        if separation.sq_len() == 0.0 {
            true
        } else {
            let closest_corner_axis = separation.unit();
            intersects_on_axis_moving(triangle, circle, dv, &closest_corner_axis)
        }
    }
}

pub fn collides(
    triangle: &Triangle,
    circle: &Circle,
    dv: &(f64, f64),
) -> Option<Collision> {
    if !intersects_moving(triangle, circle, dv) || intersects(triangle, circle) {
        return None;
    }

    triangle.normals.iter()
    .map(|axis| collision_on_axis(triangle, circle, dv, axis))
    .flatten()
    .sorted_by(|a, b| { a.dt.total_cmp(&b.dt).reverse() }) // latest collision first
    .map(|collision| corner_collision(triangle, circle, dv, collision))
    .next()
    .unwrap_or_else(|| corner_collision_all(triangle, circle, dv))
}

// at the point where the box/bounding box collision occurs, is the side that hits the circle
// flush to the circle, or below, or above?
// if flush, use that collision
// if not flush, then take the appropriate corner and use that for
// circle/point (ie 0-radius circle) collisions
pub fn corner_collision(
    triangle: &Triangle,
    circle: &Circle,
    dv: &(f64, f64),
    side_collision: Collision
) -> Option<Collision> {
    let dv_to_collision = dv.scale(&side_collision.dt);
    let box_at_time_of_collision = translate(triangle, &dv_to_collision);
    if hits_circle_side_flush(circle, &box_at_time_of_collision, side_collision.push) {
        Some(side_collision)
    } else {
        let nearest_corner = nearest_corner(&circle.center, &box_at_time_of_collision);
        let nearest_corner_start_point = nearest_corner.sub(&dv_to_collision);
        circle::collides(&Circle { center: nearest_corner_start_point, radius: 0.0}, circle, dv)
    }
}

fn hits_circle_side_flush(circle: &Circle, triangle_at_time_of_collision: &Triangle, push: (f64, f64)) -> bool {
    let side_axis = &push.unit().perpendicular();
    let side_projection = triangle_at_time_of_collision.project(side_axis);
    let center_proj = circle.center.dot(side_axis);
    side_projection.min <= center_proj && center_proj <= side_projection.max
}

fn corner_collision_all(triangle: &Triangle, circle: &Circle, dv: &(f64, f64)) -> Option<Collision> {
    triangle.vertices
        .iter()
        .map(|&(x, y)| circle::collides(&Circle { center: (x, y), radius: 0.0}, circle, dv))
        .flatten()
        .sorted_by(|a, b| { a.dt.total_cmp(&b.dt)})
        .collect::<VecDeque<Collision>>()
        .pop_front()
}

pub fn nearest_corner(point: &(f64, f64), triangle: &Triangle) -> (f64, f64) {
    nearest(point, &triangle.vertices)
}