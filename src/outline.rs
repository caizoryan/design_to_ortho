use bevy::prelude::*;

use crate::Bounds;
// make a block that will take Bounds as input and output a
// series of meshes that create outline of a cuboid

pub fn make_outline_block(bounds: &Bounds) -> Vec<Mesh> {
    const OUTLINE_WIDTH: f32 = 0.01;

    // front top
    let c1 = (
        Vec3::new(
            bounds.min.x,
            bounds.min.y - OUTLINE_WIDTH,
            bounds.max.z + OUTLINE_WIDTH,
        ) * 3.,
        Vec3::new(
            bounds.max.x,
            bounds.min.y + OUTLINE_WIDTH,
            bounds.max.z - OUTLINE_WIDTH,
        ) * 3.,
    );

    // back top
    let c2 = (
        Vec3::new(
            bounds.min.x,
            bounds.min.y - OUTLINE_WIDTH,
            bounds.min.z + OUTLINE_WIDTH,
        ) * 3.,
        Vec3::new(
            bounds.max.x,
            bounds.min.y + OUTLINE_WIDTH,
            bounds.min.z - OUTLINE_WIDTH,
        ) * 3.,
    );

    // front bottom
    let c3 = (
        Vec3::new(
            bounds.min.x,
            bounds.max.y - OUTLINE_WIDTH,
            bounds.max.z + OUTLINE_WIDTH,
        ) * 3.,
        Vec3::new(
            bounds.max.x,
            bounds.max.y + OUTLINE_WIDTH,
            bounds.max.z - OUTLINE_WIDTH,
        ) * 3.,
    );

    // back bottom
    let c4 = (
        Vec3::new(
            bounds.min.x,
            bounds.max.y - OUTLINE_WIDTH,
            bounds.min.z + OUTLINE_WIDTH,
        ) * 3.,
        Vec3::new(
            bounds.max.x,
            bounds.max.y + OUTLINE_WIDTH,
            bounds.min.z - OUTLINE_WIDTH,
        ) * 3.,
    );

    // left top
    let c5 = (
        Vec3::new(
            bounds.min.x - OUTLINE_WIDTH,
            bounds.max.y + OUTLINE_WIDTH,
            bounds.min.z,
        ) * 3.,
        Vec3::new(
            bounds.min.x + OUTLINE_WIDTH,
            bounds.max.y - OUTLINE_WIDTH,
            bounds.max.z,
        ) * 3.,
    );
    // left bottom
    let c6 = (
        Vec3::new(
            bounds.min.x - OUTLINE_WIDTH,
            bounds.min.y + OUTLINE_WIDTH,
            bounds.min.z,
        ) * 3.,
        Vec3::new(
            bounds.min.x + OUTLINE_WIDTH,
            bounds.min.y - OUTLINE_WIDTH,
            bounds.max.z,
        ) * 3.,
    );
    // right top
    let c7 = (
        Vec3::new(
            bounds.max.x - OUTLINE_WIDTH,
            bounds.max.y + OUTLINE_WIDTH,
            bounds.min.z,
        ) * 3.,
        Vec3::new(
            bounds.max.x + OUTLINE_WIDTH,
            bounds.max.y - OUTLINE_WIDTH,
            bounds.max.z,
        ) * 3.,
    );

    // right bottom
    let c8 = (
        Vec3::new(
            bounds.max.x - OUTLINE_WIDTH,
            bounds.min.y + OUTLINE_WIDTH,
            bounds.min.z,
        ) * 3.,
        Vec3::new(
            bounds.max.x + OUTLINE_WIDTH,
            bounds.min.y - OUTLINE_WIDTH,
            bounds.max.z,
        ) * 3.,
    );

    // left front edge
    let c9 = (
        Vec3::new(
            bounds.min.x - OUTLINE_WIDTH,
            bounds.min.y,
            bounds.max.z + OUTLINE_WIDTH,
        ) * 3.,
        Vec3::new(
            bounds.min.x + OUTLINE_WIDTH,
            bounds.max.y,
            bounds.max.z - OUTLINE_WIDTH,
        ) * 3.,
    );

    // left back edge
    let c10 = (
        Vec3::new(
            bounds.min.x - OUTLINE_WIDTH,
            bounds.min.y,
            bounds.min.z + OUTLINE_WIDTH,
        ) * 3.,
        Vec3::new(
            bounds.min.x + OUTLINE_WIDTH,
            bounds.max.y,
            bounds.min.z - OUTLINE_WIDTH,
        ) * 3.,
    );

    // right front edge
    let c11 = (
        Vec3::new(
            bounds.max.x - OUTLINE_WIDTH,
            bounds.min.y,
            bounds.max.z + OUTLINE_WIDTH,
        ) * 3.,
        Vec3::new(
            bounds.max.x + OUTLINE_WIDTH,
            bounds.max.y,
            bounds.max.z - OUTLINE_WIDTH,
        ) * 3.,
    );

    // right back edge
    let c12 = (
        Vec3::new(
            bounds.max.x - OUTLINE_WIDTH,
            bounds.min.y,
            bounds.min.z + OUTLINE_WIDTH,
        ) * 3.,
        Vec3::new(
            bounds.max.x + OUTLINE_WIDTH,
            bounds.max.y,
            bounds.min.z - OUTLINE_WIDTH,
        ) * 3.,
    );

    let mut v = Vec::new();
    v.push(Mesh::from(shape::Box::from_corners(c1.0, c1.1)));
    v.push(Mesh::from(shape::Box::from_corners(c2.0, c2.1)));
    v.push(Mesh::from(shape::Box::from_corners(c3.0, c3.1)));
    v.push(Mesh::from(shape::Box::from_corners(c4.0, c4.1)));
    v.push(Mesh::from(shape::Box::from_corners(c5.0, c5.1)));
    v.push(Mesh::from(shape::Box::from_corners(c6.0, c6.1)));
    v.push(Mesh::from(shape::Box::from_corners(c7.0, c7.1)));
    v.push(Mesh::from(shape::Box::from_corners(c8.0, c8.1)));
    v.push(Mesh::from(shape::Box::from_corners(c9.0, c9.1)));
    v.push(Mesh::from(shape::Box::from_corners(c10.0, c10.1)));
    v.push(Mesh::from(shape::Box::from_corners(c11.0, c11.1)));
    v.push(Mesh::from(shape::Box::from_corners(c12.0, c12.1)));

    v
}
