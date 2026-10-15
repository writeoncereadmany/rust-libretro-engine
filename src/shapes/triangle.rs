use crate::shapes::collision::Collision;
use crate::shapes::projection::{collision_on_axis, intersects_on_axis, intersects_on_axis_moving, Projection, Projects};
use crate::shapes::vec2d::Vec2d;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [(f64, f64); 3],
    pub normals: [(f64, f64); 3],
}

fn normals(vertices: [(f64, f64); 3]) -> [(f64, f64); 3] {
    [
        vertices[0].sub(&vertices[1]).unit(),
        vertices[1].sub(&vertices[2]).unit(),
        vertices[2].sub(&vertices[0]).unit(),
    ]
}

impl Triangle {
    pub fn new(vertices: [(f64, f64); 3]) -> Self {
        Triangle {
            vertices,
            normals: normals(vertices),
        }
    }
}

pub fn translate(other : &Triangle, (dx, dy): &(f64, f64)) -> Triangle {
    Triangle {
        vertices: other.vertices.map(|(x, y)| (x + dx, y + dy)),
        normals: other.normals
    }
}

pub fn intersects(tri1: &Triangle, tri2: &Triangle) -> bool {
    tri1.normals.iter().chain(tri2.normals.iter())
        .all(|normal| intersects_on_axis(tri1, tri2, &normal))
}

pub fn intersects_moving(tri1: &Triangle, tri2: &Triangle, dv: &(f64, f64)) -> bool {
    if dv.sq_len() == 0.0 {
        return intersects(tri1, tri2)
    }

    tri1.normals.iter().chain(tri2.normals.iter())
        .all(|normal| intersects_on_axis_moving(tri1, tri2, dv, &normal)) && {
        let normal_dv = dv.perpendicular().unit();
        intersects_on_axis_moving(tri1, tri2, dv, &normal_dv)
    }
}

pub fn collides(
    triangle1: &Triangle,
    triangle2: &Triangle,
    dv: &(f64, f64),
) -> Option<Collision> {
    if !intersects_moving(triangle1, triangle2, dv) || intersects(triangle1, triangle2) {
        return None;
    }

    triangle1.normals.iter().chain(triangle2.normals.iter())
        .filter_map(|axis| collision_on_axis(triangle1, triangle2, dv, axis))
        .sorted_by(|c1, c2| f64::total_cmp(&c1.dt, &c2.dt).reverse())
        .next()

}

impl Projects for Triangle {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        let projection_0 = self.vertices[0].dot(axis);
        let projection_1 = self.vertices[1].dot(axis);
        let projection_2 = self.vertices[2].dot(axis);
        Projection {
            min : projection_0.min(projection_1).min(projection_2),
            max : projection_0.max(projection_1).max(projection_2)
        }
    }
}