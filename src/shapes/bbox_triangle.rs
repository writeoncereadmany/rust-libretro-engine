use itertools::Itertools;
use crate::shapes::bbox::BBox;
use crate::shapes::collision::Collision;
use crate::shapes::projection::{collision_on_axis, intersects_on_axis, intersects_on_axis_moving};
use crate::shapes::triangle::Triangle;
use crate::shapes::vec2d::{UNIT_X, UNIT_Y};

pub fn intersects(bbox: &BBox, triangle: &Triangle) -> bool {
    [UNIT_X, UNIT_Y].iter().chain(triangle.normals.iter())
        .all(|axis| intersects_on_axis(bbox, triangle, axis))
}

pub fn intersects_moving(bbox: &BBox, triangle: &Triangle, dv: &(f64, f64)) -> bool {
    [UNIT_X, UNIT_Y].iter().chain(triangle.normals.iter())
        .all(|axis| intersects_on_axis_moving(bbox, triangle, dv, axis))
}

pub fn collides(
    bbox: &BBox,
    triangle: &Triangle,
    dv: &(f64, f64),
) -> Option<Collision> {
    if !intersects_moving(bbox, triangle, dv) || intersects(bbox, triangle) {
        return None;
    }

    [UNIT_X, UNIT_Y].iter().chain(triangle.normals.iter())
        .filter_map(|axis| collision_on_axis(bbox, triangle, dv, axis))
        .sorted_by(|c1, c2| f64::total_cmp(&c1.dt, &c2.dt).reverse())
        .next()
}