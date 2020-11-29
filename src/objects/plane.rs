use crate::core::matrix::Matrix4x4;
use crate::ray_tracing::material::Material;

#[derive(Debug, PartialEq)]
pub struct Plane {
    transform: Matrix4x4,
    material: Material,
}